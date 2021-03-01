package ard

import (
	"github.com/tliron/yamlkeys"
)

func MapsToStringMaps(value Value) (Value, bool) {
	switch value_ := value.(type) {
	case Map:
		return MapToStringMap(value_), true

	case StringMap:
		stringMap := make(StringMap)
		changed := false
		for key, element := range value_ {
			var changed_ bool
			if element, changed_ = MapsToStringMaps(element); changed_ {
				changed = true
			}
			stringMap[key] = element
		}
		if changed {
			return stringMap, true
		}

	case List:
		list := make(List, len(value_))
		changed := false
		for index, element := range value_ {
			var changed_ bool
			if element, changed_ = MapsToStringMaps(element); changed_ {
				changed = true
			}
			list[index] = element
		}
		if changed {
			return list, true
		}
	}

	return value, false
}

// Ensure data adheres to map[string]interface{}
// (JSON encoding does not support map[interface{}]interface{})
func EnsureStringMaps(stringMap StringMap) StringMap {
	stringMap_, _ := MapsToStringMaps(stringMap)
	return stringMap_.(StringMap)
}

// Recursive
func StringMapToMap(stringMap StringMap) Map {
	map_ := make(Map)
	for key, value := range stringMap {
		map_[key], _ = Normalize(value)
	}
	return map_
}

// Recursive
func MapToStringMap(map_ Map) StringMap {
	stringMap := make(StringMap)
	for key, value := range map_ {
		stringMap[yamlkeys.KeyString(key)], _ = MapsToStringMaps(value)
	}
	return stringMap
}
