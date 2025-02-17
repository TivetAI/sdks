// This file was auto-generated by Fern from our API Definition.

package actor

import (
	json "encoding/json"
	fmt "fmt"
	uuid "github.com/google/uuid"
	core "sdk/core"
	upload "sdk/upload"
)

type CompleteBuildRequestQuery struct {
	Project     *string `json:"-"`
	Environment *string `json:"-"`
}

type GetBuildRequestQuery struct {
	Project     *string `json:"-"`
	Environment *string `json:"-"`
}

type ListBuildsRequestQuery struct {
	Project     *string `json:"-"`
	Environment *string `json:"-"`
	TagsJson    *string `json:"-"`
}

type PatchBuildTagsRequestQuery struct {
	Project     *string                `json:"-"`
	Environment *string                `json:"-"`
	Body        *PatchBuildTagsRequest `json:"-"`
}

func (p *PatchBuildTagsRequestQuery) UnmarshalJSON(data []byte) error {
	body := new(PatchBuildTagsRequest)
	if err := json.Unmarshal(data, &body); err != nil {
		return err
	}
	p.Body = body
	return nil
}

func (p *PatchBuildTagsRequestQuery) MarshalJSON() ([]byte, error) {
	return json.Marshal(p.Body)
}

type PrepareBuildRequestQuery struct {
	Project     *string              `json:"-"`
	Environment *string              `json:"-"`
	Body        *PrepareBuildRequest `json:"-"`
}

func (p *PrepareBuildRequestQuery) UnmarshalJSON(data []byte) error {
	body := new(PrepareBuildRequest)
	if err := json.Unmarshal(data, &body); err != nil {
		return err
	}
	p.Body = body
	return nil
}

func (p *PrepareBuildRequestQuery) MarshalJSON() ([]byte, error) {
	return json.Marshal(p.Body)
}

type GetBuildResponse struct {
	Build *Build `json:"build,omitempty"`

	_rawJSON json.RawMessage
}

func (g *GetBuildResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler GetBuildResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*g = GetBuildResponse(value)
	g._rawJSON = json.RawMessage(data)
	return nil
}

func (g *GetBuildResponse) String() string {
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

type ListBuildsResponse struct {
	// A list of builds for the project associated with the token.
	Builds []*Build `json:"builds,omitempty"`

	_rawJSON json.RawMessage
}

func (l *ListBuildsResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler ListBuildsResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*l = ListBuildsResponse(value)
	l._rawJSON = json.RawMessage(data)
	return nil
}

func (l *ListBuildsResponse) String() string {
	if len(l._rawJSON) > 0 {
		if value, err := core.StringifyJSON(l._rawJSON); err == nil {
			return value
		}
	}
	if value, err := core.StringifyJSON(l); err == nil {
		return value
	}
	return fmt.Sprintf("%#v", l)
}

type PatchBuildTagsRequest struct {
	Tags interface{} `json:"tags,omitempty"`
	// **Deprecated**
	// Removes the given tag keys from all other builds.
	ExclusiveTags []string `json:"exclusive_tags,omitempty"`

	_rawJSON json.RawMessage
}

func (p *PatchBuildTagsRequest) UnmarshalJSON(data []byte) error {
	type unmarshaler PatchBuildTagsRequest
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*p = PatchBuildTagsRequest(value)
	p._rawJSON = json.RawMessage(data)
	return nil
}

func (p *PatchBuildTagsRequest) String() string {
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

type PatchBuildTagsResponse struct {
	_rawJSON json.RawMessage
}

func (p *PatchBuildTagsResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler PatchBuildTagsResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*p = PatchBuildTagsResponse(value)
	p._rawJSON = json.RawMessage(data)
	return nil
}

func (p *PatchBuildTagsResponse) String() string {
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

type PrepareBuildRequest struct {
	// A tag given to the project build.
	ImageTag    *string             `json:"image_tag,omitempty"`
	ImageFile   *upload.PrepareFile `json:"image_file,omitempty"`
	Kind        *BuildKind          `json:"kind,omitempty"`
	Compression *BuildCompression   `json:"compression,omitempty"`

	_rawJSON json.RawMessage
}

func (p *PrepareBuildRequest) UnmarshalJSON(data []byte) error {
	type unmarshaler PrepareBuildRequest
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*p = PrepareBuildRequest(value)
	p._rawJSON = json.RawMessage(data)
	return nil
}

func (p *PrepareBuildRequest) String() string {
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

type PrepareBuildResponse struct {
	Build             uuid.UUID                  `json:"build"`
	PresignedRequests []*upload.PresignedRequest `json:"presigned_requests,omitempty"`

	_rawJSON json.RawMessage
}

func (p *PrepareBuildResponse) UnmarshalJSON(data []byte) error {
	type unmarshaler PrepareBuildResponse
	var value unmarshaler
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	*p = PrepareBuildResponse(value)
	p._rawJSON = json.RawMessage(data)
	return nil
}

func (p *PrepareBuildResponse) String() string {
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
