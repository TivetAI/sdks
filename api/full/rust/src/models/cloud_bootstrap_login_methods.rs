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
pub struct CloudBootstrapLoginMethods {
	#[serde(rename = "email")]
	pub email: bool,
	#[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
	pub access_token: Option<bool>,
}

impl CloudBootstrapLoginMethods {
	pub fn new(email: bool) -> CloudBootstrapLoginMethods {
		CloudBootstrapLoginMethods {
			email,
			access_token: None,
		}
	}
}
