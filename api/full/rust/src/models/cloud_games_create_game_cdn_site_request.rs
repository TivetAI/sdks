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
pub struct CloudGamesCreateGameCdnSiteRequest {
	/// Represent a resource's readable display name.
	#[serde(rename = "display_name")]
	pub display_name: String,
	/// A list of files preparing to upload.
	#[serde(rename = "files")]
	pub files: Vec<crate::models::UploadPrepareFile>,
}

impl CloudGamesCreateGameCdnSiteRequest {
	pub fn new(
		display_name: String,
		files: Vec<crate::models::UploadPrepareFile>,
	) -> CloudGamesCreateGameCdnSiteRequest {
		CloudGamesCreateGameCdnSiteRequest {
			display_name,
			files,
		}
	}
}
