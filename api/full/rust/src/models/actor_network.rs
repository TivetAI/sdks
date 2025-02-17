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
pub struct ActorNetwork {
	#[serde(rename = "mode")]
	pub mode: crate::models::ActorNetworkMode,
	#[serde(rename = "ports")]
	pub ports: ::std::collections::HashMap<String, crate::models::ActorPort>,
}

impl ActorNetwork {
	pub fn new(
		mode: crate::models::ActorNetworkMode,
		ports: ::std::collections::HashMap<String, crate::models::ActorPort>,
	) -> ActorNetwork {
		ActorNetwork { mode, ports }
	}
}
