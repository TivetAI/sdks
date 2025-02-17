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
pub struct ActorCreateActorPortRequest {
	#[serde(rename = "protocol")]
	pub protocol: crate::models::ActorPortProtocol,
	#[serde(rename = "internal_port", skip_serializing_if = "Option::is_none")]
	pub internal_port: Option<i32>,
	#[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
	pub routing: Option<Box<crate::models::ActorPortRouting>>,
}

impl ActorCreateActorPortRequest {
	pub fn new(protocol: crate::models::ActorPortProtocol) -> ActorCreateActorPortRequest {
		ActorCreateActorPortRequest {
			protocol,
			internal_port: None,
			routing: None,
		}
	}
}
