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
pub struct MatchmakerListRegionsResponse {
	#[serde(rename = "regions")]
	pub regions: Vec<crate::models::MatchmakerRegionInfo>,
}

impl MatchmakerListRegionsResponse {
	pub fn new(regions: Vec<crate::models::MatchmakerRegionInfo>) -> MatchmakerListRegionsResponse {
		MatchmakerListRegionsResponse { regions }
	}
}
