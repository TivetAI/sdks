/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GroupHandle : A group handle.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GroupHandle {
    /// The URL of this group's avatar image
    #[serde(rename = "avatar_url", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// Represent a resource's readable display name.
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "external")]
    pub external: Box<crate::models::GroupExternalLinks>,
    #[serde(rename = "group_id")]
    pub group_id: uuid::Uuid,
    /// Whether or not this group is a developer group.
    #[serde(rename = "is_developer", skip_serializing_if = "Option::is_none")]
    pub is_developer: Option<bool>,
}

impl GroupHandle {
    /// A group handle.
    pub fn new(display_name: String, external: crate::models::GroupExternalLinks, group_id: uuid::Uuid) -> GroupHandle {
        GroupHandle {
            avatar_url: None,
            display_name,
            external: Box::new(external),
            group_id,
            is_developer: None,
        }
    }
}


