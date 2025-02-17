// This file was auto-generated by Fern from our API Definition.

package identity

import (
	json "encoding/json"
	fmt "fmt"
	uuid "github.com/google/uuid"
	sdk "sdk"
	identity "sdk/common/identity"
	core "sdk/core"
	upload "sdk/upload"
)

type GetHandlesResponse struct {
	Identities []*identity.Handle `json:"identities,omitempty"`
	Watch      *sdk.WatchResponse `json:"watch,omitempty"`

	_rawJSON json.RawMessage
}

func (g *GetHandlesResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler GetHandlesResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*g = GetHandlesResponse(value)
	g._rawJSON = json.RawMessage(data)
	return nil
}

func (g *GetHandlesResponse) String() string {
	if len(g._rawJSON) > 0 {
		if value, err := core.StringifyJSON(g._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(g); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", g)
}

type GetProfileResponse struct {
	Identity *Profile           `json:"identity,omitempty"`
	Watch    *sdk.WatchResponse `json:"watch,omitempty"`

	_rawJSON json.RawMessage
}

func (g *GetProfileResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler GetProfileResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*g = GetProfileResponse(value)
	g._rawJSON = json.RawMessage(data)
	return nil
}

func (g *GetProfileResponse) String() string {
	if len(g._rawJSON) > 0 {
		if value, err := core.StringifyJSON(g._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(g); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", g)
}

type GetSummariesResponse struct {
	Identities []*Summary         `json:"identities,omitempty"`
	Watch      *sdk.WatchResponse `json:"watch,omitempty"`

	_rawJSON json.RawMessage
}

func (g *GetSummariesResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler GetSummariesResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*g = GetSummariesResponse(value)
	g._rawJSON = json.RawMessage(data)
	return nil
}

func (g *GetSummariesResponse) String() string {
	if len(g._rawJSON) > 0 {
		if value, err := core.StringifyJSON(g._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(g); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", g)
}

type PrepareAvatarUploadResponse struct {
	UploadId         uuid.UUID                `json:"upload_id"`
	PresignedRequest *upload.PresignedRequest `json:"presigned_request,omitempty"`

	_rawJSON json.RawMessage
}

func (p *PrepareAvatarUploadResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler PrepareAvatarUploadResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*p = PrepareAvatarUploadResponse(value)
	p._rawJSON = json.RawMessage(data)
	return nil
}

func (p *PrepareAvatarUploadResponse) String() string {
	if len(p._rawJSON) > 0 {
		if value, err := core.StringifyJSON(p._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(p); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", p)
}

type SetupResponse struct {
	// Token used to authenticate the identity.
	// Should be stored somewhere permanent.
	// Pass this to `tivet.api.identity#Setup$existing_identity_token` next time `tivet.api.identity#Setup` is called.
	// Token has a 90 day TTL.
	// This means that if `tivet.api.identity#Setup` is not called again within 90 days, the token will no longer be valid.
	// If this happens, the user can recover their account through the linking process (see `tivet.api.identity#PrepareGameLink`).
	// This token should be stored locally and never sent to a server or another device.
	// If this token is compromised, anyone with access to this token has control of the identity.
	IdentityToken         sdk.Jwt       `json:"identity_token"`
	IdentityTokenExpireTs sdk.Timestamp `json:"identity_token_expire_ts"`
	// Information about the identity that was just authenticated.
	Identity *Profile  `json:"identity,omitempty"`
	GameId   uuid.UUID `json:"game_id"`

	_rawJSON json.RawMessage
}

func (s *SetupResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler SetupResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*s = SetupResponse(value)
	s._rawJSON = json.RawMessage(data)
	return nil
}

func (s *SetupResponse) String() string {
	if len(s._rawJSON) > 0 {
		if value, err := core.StringifyJSON(s._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(s); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", s)
}
