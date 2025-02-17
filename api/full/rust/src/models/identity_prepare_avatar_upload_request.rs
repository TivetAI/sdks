/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentityPrepareAvatarUploadRequest {
	#[serde(rename = "path")]
	pub path: String,
	/// See https://www.iana.org/assignments/media-types/media-types.xhtml
	#[serde(rename = "mime")]
	pub mime: String,
	#[serde(rename = "content_length")]
	pub content_length: i64,
}

impl IdentityPrepareAvatarUploadRequest {
	pub fn new(
		path: String,
		mime: String,
		content_length: i64,
	) -> IdentityPrepareAvatarUploadRequest {
		IdentityPrepareAvatarUploadRequest {
			path,
			mime,
			content_length,
		}
	}
}
