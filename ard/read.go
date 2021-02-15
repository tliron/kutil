package ard

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"

	"github.com/beevik/etree"
	"github.com/tliron/yamlkeys"
	"gopkg.in/yaml.v3"
)

func Read(reader io.Reader, format string, locate bool) (Value, Locator, error) {
	switch format {
	case "yaml", "":
		return ReadYAML(reader, locate)

	case "json":
		return ReadJSON(reader, locate)

	case "cjson":
		return ReadCompatibleJSON(reader, locate)

	case "xml":
		return ReadCompatibleXML(reader, locate)

	default:
		return nil, nil, fmt.Errorf("unsupported format: %q", format)
	}
}

func ReadYAML(reader io.Reader, locate bool) (Value, Locator, error) {
	var node yaml.Node
	decoder := yaml.NewDecoder(reader)
	if err := decoder.Decode(&node); err == nil {
		if value, err := yamlkeys.DecodeNode(&node); err == nil {
			var locator Locator
			if locate {
				locator = NewYAMLLocator(&node)
			}
			// We do not need to call EnsureMaps because yamlkeys takes care of it
			return value, locator, nil
		} else {
			return nil, nil, err
		}
	} else {
		return nil, nil, yamlkeys.WrapWithDecodeError(err)
	}
}

func ReadAllYAML(reader io.Reader) (List, error) {
	return yamlkeys.DecodeAll(reader)
}

func ReadJSON(reader io.Reader, locate bool) (Value, Locator, error) {
	var value Value
	decoder := json.NewDecoder(reader)
	if err := decoder.Decode(&value); err == nil {
		return EnsureMaps(value), nil, nil
	} else {
		return nil, nil, err
	}
}

func ReadCompatibleJSON(reader io.Reader, locate bool) (Value, Locator, error) {
	var value Value
	decoder := json.NewDecoder(reader)
	if err := decoder.Decode(&value); err == nil {
		// We do not need to call EnsureMaps because FromCompatibleJSON takes care of it
		return FromCompatibleJSON(value), nil, nil
	} else {
		return nil, nil, err
	}
}

func ReadCompatibleXML(reader io.Reader, locate bool) (Value, Locator, error) {
	document := etree.NewDocument()
	if _, err := document.ReadFrom(reader); err == nil {
		elements := document.ChildElements()
		if len(elements) == 1 {
			// We do not need to call EnsureMaps because FromCompatibleXML takes care of it
			value, err := FromCompatibleXML(elements[0])
			return value, nil, err
		} else {
			return nil, nil, errors.New("unsupported XML")
		}
	} else {
		return nil, nil, err
	}
}
