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
pub struct CloudGamesGameLogoUploadPrepareRequest {
	/// The path/filename of the game logo.
	#[serde(rename = "path")]
	pub path: String,
	/// The MIME type of the game logo.
	#[serde(rename = "mime", skip_serializing_if = "Option::is_none")]
	pub mime: Option<String>,
	/// Unsigned 64 bit integer.
	#[serde(rename = "content_length")]
	pub content_length: i64,
}

impl CloudGamesGameLogoUploadPrepareRequest {
	pub fn new(path: String, content_length: i64) -> CloudGamesGameLogoUploadPrepareRequest {
		CloudGamesGameLogoUploadPrepareRequest {
			path,
			mime: None,
			content_length,
		}
	}
}
