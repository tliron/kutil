package ard

//
// Node
//

type Node struct {
	Data any

	container *Node
	key       string
	allowNil  bool
}

func NewNode(data any) *Node {
	return &Node{data, nil, "", false}
}

var NoNode = &Node{nil, nil, "", false}

func (self *Node) AllowNil() *Node {
	if self != NoNode {
		return &Node{self.Data, self.container, self.key, true}
	}
	return NoNode
}

func (self *Node) Get(key string) *Node {
	if self != NoNode {
		if data_, ok := self.Data.(StringMap); ok {
			if value, ok := data_[key]; ok {
				return &Node{value, self, key, self.allowNil}
			}
		} else if data_, ok := self.Data.(Map); ok {
			if value, ok := data_[key]; ok {
				return &Node{value, self, key, self.allowNil}
			}
		}
	}
	return NoNode
}

func (self *Node) Put(key string, value any) bool {
	if self != NoNode {
		if data_, ok := self.Data.(StringMap); ok {
			data_[key] = value
			return true
		} else if data_, ok := self.Data.(Map); ok {
			data_[key] = value
			return true
		}
	}
	return false
}

func (self *Node) Append(value any) bool {
	if self != NoNode {
		if data_, ok := self.Data.(List); ok {
			self.container.Put(self.key, append(data_, value))
			return true
		}
	}
	return false
}

func (self *Node) Bytes() ([]byte, bool) {
	if self != NoNode {
		if self.allowNil && (self.Data == nil) {
			return nil, true
		}
		value, ok := self.Data.([]byte)
		return value, ok
	}
	return nil, false
}

func (self *Node) String() (string, bool) {
	if self != NoNode {
		if self.allowNil && (self.Data == nil) {
			return "", true
		}
		value, ok := self.Data.(string)
		return value, ok
	}
	return "", false
}

func (self *Node) Integer() (int64, bool) {
	if self != NoNode {
		if self.allowNil && (self.Data == nil) {
			return 0, true
		}
		switch value := self.Data.(type) {
		case int64:
			return value, true
		case int32:
			return int64(value), true
		case int16:
			return int64(value), true
		case int8:
			return int64(value), true
		case int:
			return int64(value), true
		}
	}
	return 0, false
}

func (self *Node) NumberAsInteger() (int64, bool) {
	if self != NoNode {
		if self.allowNil && (self.Data == nil) {
			return 0, true
		}
		switch value := self.Data.(type) {
		case int64:
			return value, true
		case int32:
			return int64(value), true
		case int16:
			return int64(value), true
		case int8:
			return int64(value), true
		case int:
			return int64(value), true
		case uint64:
			return int64(value), true
		case uint32:
			return int64(value), true
		case uint16:
			return int64(value), true
		case uint8:
			return int64(value), true
		case uint:
			return int64(value), true
		case float64:
			return int64(value), true
		case float32:
			return int64(value), true
		}
	}
	return 0, false
}

func (self *Node) UnsignedInteger() (uint64, bool) {
	if self != NoNode {
		if self.allowNil && (self.Data == nil) {
			return 0, true
		}
		switch value := self.Data.(type) {
		case uint64:
			return value, true
		case uint32:
			return uint64(value), true
		case uint16:
			return uint64(value), true
		case uint8:
			return uint64(value), true
		case uint:
			return uint64(value), true
		}
	}
	return 0, false
}

func (self *Node) Float() (float64, bool) {
	if self != NoNode {
		if self.allowNil && (self.Data == nil) {
			return 0.0, true
		}
		switch value := self.Data.(type) {
		case float64:
			return value, true
		case float32:
			return float64(value), true
		}
	}
	return 0.0, false
}

func (self *Node) NumberAsFloat() (float64, bool) {
	if self != NoNode {
		if self.allowNil && (self.Data == nil) {
			return 0.0, true
		}
		switch value := self.Data.(type) {
		case float64:
			return value, true
		case float32:
			return float64(value), true
		case int64:
			return float64(value), true
		case int32:
			return float64(value), true
		case int16:
			return float64(value), true
		case int8:
			return float64(value), true
		case int:
			return float64(value), true
		case uint64:
			return float64(value), true
		case uint32:
			return float64(value), true
		case uint16:
			return float64(value), true
		case uint8:
			return float64(value), true
		case uint:
			return float64(value), true
		}
	}
	return 0.0, false
}

func (self *Node) Boolean() (bool, bool) {
	if self != NoNode {
		if self.allowNil && (self.Data == nil) {
			return false, true
		}
		if value, ok := self.Data.(bool); ok {
			return value, true
		}
	}
	return false, false
}

func (self *Node) StringMap() (StringMap, bool) {
	if self != NoNode {
		if self.allowNil && (self.Data == nil) {
			return make(StringMap), true
		}
		value, ok := self.Data.(StringMap)
		return value, ok
	}
	return nil, false
}

func (self *Node) Map() (Map, bool) {
	if self != NoNode {
		if self.allowNil && (self.Data == nil) {
			return make(Map), true
		}
		value, ok := self.Data.(Map)
		return value, ok
	}
	return nil, false
}

func (self *Node) List() (List, bool) {
	if self != NoNode {
		if self.allowNil && (self.Data == nil) {
			return nil, true
		}
		value, ok := self.Data.(List)
		return value, ok
	}
	return nil, false
}

func (self *Node) StringList() ([]string, bool) {
	if self != NoNode {
		if self.allowNil && (self.Data == nil) {
			return nil, true
		}
		if value, ok := self.Data.(List); ok {
			list := make([]string, len(value))
			for index, element := range value {
				if list[index], ok = element.(string); !ok {
					return nil, false
				}
			}
			return list, true
		} else {
			return nil, false
		}
	}
	return nil, false
}
