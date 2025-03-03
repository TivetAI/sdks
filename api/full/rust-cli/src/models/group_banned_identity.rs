/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GroupBannedIdentity : A banned identity.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GroupBannedIdentity {
    /// RFC3339 timestamp
    #[serde(rename = "ban_ts")]
    pub ban_ts: String,
    #[serde(rename = "identity")]
    pub identity: Box<crate::models::IdentityHandle>,
}

impl GroupBannedIdentity {
    /// A banned identity.
    pub fn new(ban_ts: String, identity: crate::models::IdentityHandle) -> GroupBannedIdentity {
        GroupBannedIdentity {
            ban_ts,
            identity: Box::new(identity),
        }
    }
}


