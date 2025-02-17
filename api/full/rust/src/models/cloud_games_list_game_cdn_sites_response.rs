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
pub struct CloudGamesListGameCdnSitesResponse {
	/// A list of CDN site summaries.
	#[serde(rename = "sites")]
	pub sites: Vec<crate::models::CloudCdnSiteSummary>,
}

impl CloudGamesListGameCdnSitesResponse {
	pub fn new(
		sites: Vec<crate::models::CloudCdnSiteSummary>,
	) -> CloudGamesListGameCdnSitesResponse {
		CloudGamesListGameCdnSitesResponse { sites }
	}
}
