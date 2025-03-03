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
pub struct GroupSummary {
    /// The URL of this group's avatar image.
    #[serde(rename = "avatar_url", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// Follows regex ^(?:[^\\n\\r]+\\n?|\\n){1,5}$
    #[serde(rename = "bio")]
    pub bio: String,
    /// Represent a resource's readable display name.
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "external")]
    pub external: Box<crate::models::GroupExternalLinks>,
    #[serde(rename = "group_id")]
    pub group_id: uuid::Uuid,
    /// Whether or not the current identity is a member of this group.
    #[serde(rename = "is_current_identity_member")]
    pub is_current_identity_member: bool,
    /// **Deprecated** Whether or not this group is a developer.
    #[serde(rename = "is_developer")]
    pub is_developer: bool,
    #[serde(rename = "member_count")]
    pub member_count: i32,
    #[serde(rename = "owner_identity_id")]
    pub owner_identity_id: uuid::Uuid,
    #[serde(rename = "publicity")]
    pub publicity: crate::models::GroupPublicity,
}

impl GroupSummary {
    pub fn new(bio: String, display_name: String, external: crate::models::GroupExternalLinks, group_id: uuid::Uuid, is_current_identity_member: bool, is_developer: bool, member_count: i32, owner_identity_id: uuid::Uuid, publicity: crate::models::GroupPublicity) -> GroupSummary {
        GroupSummary {
            avatar_url: None,
            bio,
            display_name,
            external: Box::new(external),
            group_id,
            is_current_identity_member,
            is_developer,
            member_count,
            owner_identity_id,
            publicity,
        }
    }
}


