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
pub struct CloudGamesValidateGameRequest {
	/// Represent a resource's readable display name.
	#[serde(rename = "display_name")]
	pub display_name: String,
	/// A human readable short identifier used to references resources. Different than a `uuid` because this is intended to be human readable. Different than `DisplayName` because this should not include special characters and be short.
	#[serde(rename = "name_id", skip_serializing_if = "Option::is_none")]
	pub name_id: Option<String>,
}

impl CloudGamesValidateGameRequest {
	pub fn new(display_name: String) -> CloudGamesValidateGameRequest {
		CloudGamesValidateGameRequest {
			display_name,
			name_id: None,
		}
	}
}
