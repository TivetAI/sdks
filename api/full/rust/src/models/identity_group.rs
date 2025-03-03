/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// IdentityGroup : A group that the given identity.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentityGroup {
	#[serde(rename = "group")]
	pub group: Box<crate::models::GroupHandle>,
}

impl IdentityGroup {
	/// A group that the given identity.
	pub fn new(group: crate::models::GroupHandle) -> IdentityGroup {
		IdentityGroup {
			group: Box::new(group),
		}
	}
}
