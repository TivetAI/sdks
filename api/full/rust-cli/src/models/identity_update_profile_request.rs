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
pub struct IdentityUpdateProfileRequest {
    #[serde(rename = "account_number", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<i32>,
    /// Follows regex ^(?:[^\\n\\r]+\\n?|\\n){1,5}$
    #[serde(rename = "bio", skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// Represent a resource's readable display name.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl IdentityUpdateProfileRequest {
    pub fn new() -> IdentityUpdateProfileRequest {
        IdentityUpdateProfileRequest {
            account_number: None,
            bio: None,
            display_name: None,
        }
    }
}


