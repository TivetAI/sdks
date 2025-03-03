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
pub struct GroupGroupSummary {
	#[serde(rename = "group_id")]
	pub group_id: uuid::Uuid,
	/// Represent a resource's readable display name.
	#[serde(rename = "display_name")]
	pub display_name: String,
	/// The URL of this group's avatar image.
	#[serde(rename = "avatar_url", skip_serializing_if = "Option::is_none")]
	pub avatar_url: Option<String>,
	#[serde(rename = "external")]
	pub external: Box<crate::models::GroupExternalLinks>,
	/// **Deprecated** Whether or not this group is a developer.
	#[serde(rename = "is_developer")]
	pub is_developer: bool,
	/// Follows regex ^(?:[^\\n\\r]+\\n?|\\n){1,5}$
	#[serde(rename = "bio")]
	pub bio: String,
	/// Whether or not the current identity is a member of this group.
	#[serde(rename = "is_current_identity_member")]
	pub is_current_identity_member: bool,
	#[serde(rename = "publicity")]
	pub publicity: crate::models::GroupPublicity,
	#[serde(rename = "member_count")]
	pub member_count: i32,
	#[serde(rename = "owner_identity_id")]
	pub owner_identity_id: uuid::Uuid,
}

impl GroupGroupSummary {
	pub fn new(
		group_id: uuid::Uuid,
		display_name: String,
		external: crate::models::GroupExternalLinks,
		is_developer: bool,
		bio: String,
		is_current_identity_member: bool,
		publicity: crate::models::GroupPublicity,
		member_count: i32,
		owner_identity_id: uuid::Uuid,
	) -> GroupGroupSummary {
		GroupGroupSummary {
			group_id,
			display_name,
			avatar_url: None,
			external: Box::new(external),
			is_developer,
			bio,
			is_current_identity_member,
			publicity,
			member_count,
			owner_identity_id,
		}
	}
}
