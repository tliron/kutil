package ard

import (
	"encoding/json"
	"strconv"

	"github.com/tliron/kutil/util"
)

/*
ARD-compatible JSON can encode features that exist in YAML but not normlly in
JSON:

1) integers and unsigned integers are preserved as distinct from floats
2) raw bytes can be encoded using Base64
3) maps are allowed to have non-string keys

This particular implementation is not designed for performance but rather for
widest compability, relying on Go's built-in JSON support or 3rd-party
implementations compatible with it.

Inspired by: https://docs.mongodb.com/manual/reference/mongodb-Compatible-json/
*/

var CompatibleJSONIntegerCode = "$ard.integer"

var CompatibleJSONUIntegerCode = "$ard.uinteger"

var CompatibleJSONBytesCode = "$ard.bytes"

var CompatibleJSONMapCode = "$ard.map"

func ToCompatibleJSON(value Value) Value {
	value, _ = TryToCompatibleJSON(value)
	return value
}

func TryToCompatibleJSON(value Value) (Value, bool) {
	switch value_ := value.(type) {
	case int:
		return CompatibleJSONInteger(int64(value_)), true
	case int64:
		return CompatibleJSONInteger(value_), true
	case int32:
		return CompatibleJSONInteger(int64(value_)), true
	case int16:
		return CompatibleJSONInteger(int64(value_)), true
	case int8:
		return CompatibleJSONInteger(int64(value_)), true

	case uint:
		return CompatibleJSONUInteger(uint64(value_)), true
	case uint64:
		return CompatibleJSONUInteger(value_), true
	case uint32:
		return CompatibleJSONUInteger(uint64(value_)), true
	case uint16:
		return CompatibleJSONUInteger(uint64(value_)), true
	case uint8:
		return CompatibleJSONUInteger(uint64(value_)), true

	case []byte:
		return CompatibleJSONBytes(value_), true

	case List:
		converted := false
		list := make(List, len(value_))

		for index, element := range value_ {
			var converted_ bool
			element, converted_ = TryToCompatibleJSON(element)
			list[index] = element
			if converted_ {
				converted = true
			}
		}

		if converted {
			return list, true
		}

	case StringMap:
		converted := false
		map_ := make(StringMap)

		for key, value__ := range value_ {
			var converted_ bool
			value__, converted_ = TryToCompatibleJSON(value__)
			map_[key] = value__
			if converted_ {
				converted = true
			}
		}

		if converted {
			return map_, true
		}

	case Map:
		// We'll build two maps at the same time, but only return one
		stringMap := make(StringMap)
		compatibleJsonMap := make(CompatibleJSONMap)
		useCompatibleJsonMap := false

		for key, value__ := range value_ {
			value__ = ToCompatibleJSON(value__)
			if key_, ok := key.(string); ok {
				compatibleJsonMap[key] = value__
				// We can stop building the stringMap if we switched to compatibleJsonMap

				if !useCompatibleJsonMap {
					stringMap[key_] = value__
				}
			} else {
				key = ToCompatibleJSON(key)
				compatibleJsonMap[key] = value__
				useCompatibleJsonMap = true
			}
		}

		if useCompatibleJsonMap {
			return compatibleJsonMap, true
		} else {
			return stringMap, true
		}
	}

	return value, true
}

func FromCompatibleJSON(value Value) Value {
	value, _ = TryFromCompatibleJSON(value)
	return value
}

func TryFromCompatibleJSON(value Value) (Value, bool) {
	switch value_ := value.(type) {
	case List:
		converted := false
		list := make(List, len(value_))

		for index, element := range value_ {
			var converted_ bool
			element, converted_ = TryFromCompatibleJSON(element)
			list[index] = element
			if converted_ {
				converted = true
			}
		}

		if converted {
			return list, true
		}

	case StringMap:
		if len(value_) == 1 {
			if integer, ok := DecodeCompatibleJSONInteger(value_); ok {
				return integer, true
			} else if uinteger, ok := DecodeCompatibleJSONUInteger(value_); ok {
				return uinteger, true
			} else if bytes, ok := DecodeCompatibleJSONBytes(value_); ok {
				return bytes, true
			} else if map_, ok := DecodeCompatibleJSONMap(value_); ok {
				return map_, true
			}
		}

		converted := false
		map_ := make(StringMap)

		for key, value__ := range value_ {
			var converted_ bool
			value__, converted_ = TryFromCompatibleJSON(value__)
			map_[key] = value__
			if converted_ {
				converted = true
			}
		}

		if converted {
			return map_, true
		}
	}

	return value, false
}

//
// CompatibleJSONInteger
//

type CompatibleJSONInteger int64

// json.Marshaler interface
func (self CompatibleJSONInteger) MarshalJSON() ([]byte, error) {
	return json.Marshal(StringMap{
		CompatibleJSONIntegerCode: strconv.FormatInt(int64(self), 10),
	})
}

func DecodeCompatibleJSONInteger(code StringMap) (int64, bool) {
	if integer, ok := code[CompatibleJSONIntegerCode]; ok {
		if integer_, ok := integer.(string); ok {
			if integer__, err := strconv.ParseInt(integer_, 10, 64); err == nil {
				return integer__, true
			}
		}
	}
	return 0, false
}

//
// CompatibleJSONUInteger
//

type CompatibleJSONUInteger uint64

// json.Marshaler interface
func (self CompatibleJSONUInteger) MarshalJSON() ([]byte, error) {
	return json.Marshal(StringMap{
		CompatibleJSONUIntegerCode: strconv.FormatUint(uint64(self), 10),
	})
}

func DecodeCompatibleJSONUInteger(code StringMap) (uint64, bool) {
	if uinteger, ok := code[CompatibleJSONUIntegerCode]; ok {
		if uinteger_, ok := uinteger.(string); ok {
			if uinteger__, err := strconv.ParseUint(uinteger_, 10, 64); err == nil {
				return uinteger__, true
			}
		}
	}
	return 0, false
}

//
// CompatibleJSONBytes
//

type CompatibleJSONBytes []byte

// json.Marshaler interface
func (self CompatibleJSONBytes) MarshalJSON() ([]byte, error) {
	return json.Marshal(StringMap{
		CompatibleJSONBytesCode: util.ToBase64(self),
	})
}

func DecodeCompatibleJSONBytes(code StringMap) ([]byte, bool) {
	if bytes, ok := code[CompatibleJSONBytesCode]; ok {
		if bytes_, ok := bytes.(string); ok {
			if bytes__, err := util.FromBase64(bytes_); err == nil {
				return bytes__, true
			}
		}
	}
	return nil, false
}

//
// CompatibleJSONMap
//

type CompatibleJSONMap Map

// json.Marshaler interface
func (self CompatibleJSONMap) MarshalJSON() ([]byte, error) {
	list := make([]CompatibleJSONMapEntry, 0, len(self))
	for key, value := range self {
		list = append(list, CompatibleJSONMapEntry{key, value})
	}

	return json.Marshal(StringMap{
		CompatibleJSONMapCode: list,
	})
}

func DecodeCompatibleJSONMap(code StringMap) (Map, bool) {
	if map_, ok := code[CompatibleJSONMapCode]; ok {
		if map__, ok := map_.(List); ok {
			r := make(Map)
			for _, entry := range map__ {
				if entry_, ok := DecodeCompatibleJSONMapEntry(entry); ok {
					r[entry_.Key] = entry_.Value
				} else {
					return nil, false
				}
			}
			return r, true
		}
	}
	return nil, false
}

//
// CompatibleJSONMapEntry
//

type CompatibleJSONMapEntry struct {
	Key   Value `json:"key"`
	Value Value `json:"value"`
}

func DecodeCompatibleJSONMapEntry(entry Value) (*CompatibleJSONMapEntry, bool) {
	if entry_, ok := entry.(StringMap); ok {
		if key, ok := entry_["key"]; ok {
			if value, ok := entry_["value"]; ok {
				return &CompatibleJSONMapEntry{
					Key:   FromCompatibleJSON(key),
					Value: FromCompatibleJSON(value),
				}, true
			}
		}
	}
	return nil, false
}
