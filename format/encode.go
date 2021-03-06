package format

import (
	"fmt"
	"strings"

	"github.com/fxamacker/cbor/v2"
	"github.com/tliron/kutil/ard"
	"github.com/tliron/kutil/util"
)

func Encode(value interface{}, format string, indent string, strict bool) (string, error) {
	switch format {
	case "yaml", "":
		return EncodeYAML(value, indent, strict)

	case "json":
		return EncodeJSON(value, indent)

	case "cjson":
		return EncodeCompatibleJSON(value, indent)

	case "xml":
		return EncodeCompatibleXML(value, indent)

	case "cbor":
		return EncodeCBOR(value)

	default:
		return "", fmt.Errorf("unsupported format: %s", format)
	}
}

func EncodeYAML(value interface{}, indent string, strict bool) (string, error) {
	var writer strings.Builder
	if err := WriteYAML(value, &writer, indent, strict); err == nil {
		return writer.String(), nil
	} else {
		return "", err
	}
}

func EncodeJSON(value interface{}, indent string) (string, error) {
	var writer strings.Builder
	if err := WriteJSON(value, &writer, indent); err == nil {
		s := writer.String()
		if indent == "" {
			// json.Encoder adds a "\n", unlike json.Marshal
			s = strings.TrimRight(s, "\n")
		}
		return s, nil
	} else {
		return "", err
	}
}

func EncodeCompatibleJSON(value interface{}, indent string) (string, error) {
	return EncodeJSON(ard.EnsureCompatibleJSON(value), indent)
}

func EncodeCompatibleXML(value interface{}, indent string) (string, error) {
	var writer strings.Builder
	if err := WriteCompatibleXML(value, &writer, indent); err == nil {
		return writer.String(), nil
	} else {
		return "", err
	}
}

// Encodes to Base64
func EncodeCBOR(value interface{}) (string, error) {
	if bytes, err := cbor.Marshal(value); err == nil {
		return util.ToBase64(bytes), nil
	} else {
		return "", err
	}
}
