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
pub struct IdentityGetSummariesResponse {
	#[serde(rename = "identities")]
	pub identities: Vec<crate::models::IdentitySummary>,
	#[serde(rename = "watch")]
	pub watch: Box<crate::models::WatchResponse>,
}

impl IdentityGetSummariesResponse {
	pub fn new(
		identities: Vec<crate::models::IdentitySummary>,
		watch: crate::models::WatchResponse,
	) -> IdentityGetSummariesResponse {
		IdentityGetSummariesResponse {
			identities,
			watch: Box::new(watch),
		}
	}
}
