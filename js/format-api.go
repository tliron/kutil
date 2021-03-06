package js

import (
	"fmt"
	"io"
	"strings"

	"github.com/beevik/etree"
	"github.com/tliron/kutil/ard"
	formatpkg "github.com/tliron/kutil/format"
	"github.com/tliron/yamlkeys"
)

type FormatAPI struct{}

func (self FormatAPI) ValidateFormat(code string, format string) error {
	return formatpkg.Validate(code, format)
}

func (self FormatAPI) Decode(code string, format string, all bool) (ard.Value, error) {
	switch format {
	case "yaml", "":
		if all {
			if value, err := yamlkeys.DecodeAll(strings.NewReader(code)); err == nil {
				value_, _ := ard.MapsToStringMaps(value)
				return value_, err
			} else {
				return nil, err
			}
		} else {
			value, _, err := ard.DecodeYAML(code, false)
			value, _ = ard.MapsToStringMaps(value)
			return value, err
		}

	case "json":
		value, _, err := ard.DecodeJSON(code, false)
		value, _ = ard.MapsToStringMaps(value)
		return value, err

	case "cjson":
		value, _, err := ard.DecodeCompatibleJSON(code, false)
		value, _ = ard.MapsToStringMaps(value)
		return value, err

	case "xml":
		value, _, err := ard.DecodeCompatibleXML(code, false)
		value, _ = ard.MapsToStringMaps(value)
		return value, err

	case "cbor":
		value, _, err := ard.DecodeCBOR(code, false)
		value, _ = ard.MapsToStringMaps(value)
		return value, err

	default:
		return nil, fmt.Errorf("unsupported format: %s", format)
	}
}

func (self FormatAPI) Encode(value interface{}, format string, indent string, writer io.Writer) (string, error) {
	if writer == nil {
		return formatpkg.Encode(value, format, indent, false)
	} else {
		err := formatpkg.Write(value, format, indent, false, writer)
		return "", err
	}
}

func (self FormatAPI) NewXMLDocument() *etree.Document {
	return etree.NewDocument()
}
