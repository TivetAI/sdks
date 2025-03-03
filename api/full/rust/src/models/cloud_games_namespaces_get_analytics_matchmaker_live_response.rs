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
pub struct CloudGamesNamespacesGetAnalyticsMatchmakerLiveResponse {
	/// A list of analytics lobby summaries.
	#[serde(rename = "lobbies")]
	pub lobbies: Vec<crate::models::CloudLobbySummaryAnalytics>,
}

impl CloudGamesNamespacesGetAnalyticsMatchmakerLiveResponse {
	pub fn new(
		lobbies: Vec<crate::models::CloudLobbySummaryAnalytics>,
	) -> CloudGamesNamespacesGetAnalyticsMatchmakerLiveResponse {
		CloudGamesNamespacesGetAnalyticsMatchmakerLiveResponse { lobbies }
	}
}
