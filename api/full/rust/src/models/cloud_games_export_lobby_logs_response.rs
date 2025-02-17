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
pub struct CloudGamesExportLobbyLogsResponse {
	/// The URL to a CSV file for the given lobby history.
	#[serde(rename = "url")]
	pub url: String,
}

impl CloudGamesExportLobbyLogsResponse {
	pub fn new(url: String) -> CloudGamesExportLobbyLogsResponse {
		CloudGamesExportLobbyLogsResponse { url }
	}
}
