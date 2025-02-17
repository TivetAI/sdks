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
pub struct ServersPort {
	#[serde(rename = "protocol")]
	pub protocol: crate::models::ServersPortProtocol,
	#[serde(rename = "internal_port", skip_serializing_if = "Option::is_none")]
	pub internal_port: Option<i32>,
	#[serde(rename = "public_hostname", skip_serializing_if = "Option::is_none")]
	pub public_hostname: Option<String>,
	#[serde(rename = "public_port", skip_serializing_if = "Option::is_none")]
	pub public_port: Option<i32>,
	#[serde(rename = "routing")]
	pub routing: Box<crate::models::ServersPortRouting>,
}

impl ServersPort {
	pub fn new(
		protocol: crate::models::ServersPortProtocol,
		routing: crate::models::ServersPortRouting,
	) -> ServersPort {
		ServersPort {
			protocol,
			internal_port: None,
			public_hostname: None,
			public_port: None,
			routing: Box::new(routing),
		}
	}
}
