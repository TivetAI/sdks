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
pub struct CloudGamesNamespacesAddNamespaceDomainRequest {
	/// A valid domain name (no protocol).
	#[serde(rename = "domain")]
	pub domain: String,
}

impl CloudGamesNamespacesAddNamespaceDomainRequest {
	pub fn new(domain: String) -> CloudGamesNamespacesAddNamespaceDomainRequest {
		CloudGamesNamespacesAddNamespaceDomainRequest { domain }
	}
}
