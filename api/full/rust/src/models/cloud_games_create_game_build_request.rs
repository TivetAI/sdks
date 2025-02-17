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
pub struct CloudGamesCreateGameBuildRequest {
	/// Represent a resource's readable display name.
	#[serde(rename = "display_name")]
	pub display_name: String,
	/// A tag given to the game build.
	#[serde(rename = "image_tag")]
	pub image_tag: String,
	#[serde(rename = "image_file")]
	pub image_file: Box<crate::models::UploadPrepareFile>,
	#[serde(rename = "multipart_upload", skip_serializing_if = "Option::is_none")]
	pub multipart_upload: Option<bool>,
	#[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
	pub kind: Option<crate::models::CloudGamesBuildKind>,
	#[serde(rename = "compression", skip_serializing_if = "Option::is_none")]
	pub compression: Option<crate::models::CloudGamesBuildCompression>,
}

impl CloudGamesCreateGameBuildRequest {
	pub fn new(
		display_name: String,
		image_tag: String,
		image_file: crate::models::UploadPrepareFile,
	) -> CloudGamesCreateGameBuildRequest {
		CloudGamesCreateGameBuildRequest {
			display_name,
			image_tag,
			image_file: Box::new(image_file),
			multipart_upload: None,
			kind: None,
			compression: None,
		}
	}
}
