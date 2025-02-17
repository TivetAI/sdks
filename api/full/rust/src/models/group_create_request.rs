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
pub struct GroupCreateRequest {
	/// Represent a resource's readable display name.
	#[serde(rename = "display_name")]
	pub display_name: String,
}

impl GroupCreateRequest {
	pub fn new(display_name: String) -> GroupCreateRequest {
		GroupCreateRequest { display_name }
	}
}
