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
pub struct ServersNetwork {
	#[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
	pub mode: Option<crate::models::ServersNetworkMode>,
	#[serde(rename = "ports")]
	pub ports: ::std::collections::HashMap<String, crate::models::ServersPort>,
}

impl ServersNetwork {
	pub fn new(
		ports: ::std::collections::HashMap<String, crate::models::ServersPort>,
	) -> ServersNetwork {
		ServersNetwork { mode: None, ports }
	}
}
