// This file was auto-generated by Fern from our API Definition.

package kv

import (
	json "encoding/json"
	fmt "fmt"
	core "sdk/core"
)

// KV configuration for a given version.
type KvConfig struct {
	_rawJSON json.RawMessage
}

func (k *KvConfig) UnmarshalJSON(data []byte) error {
	type unmarshaler KvConfig
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*k = KvConfig(value)
	k._rawJSON = json.RawMessage(data)
	return nil
}

func (k *KvConfig) String() string {
	if len(k._rawJSON) > 0 {
		if value, err := core.StringifyJSON(k._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(k); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", k)
}
