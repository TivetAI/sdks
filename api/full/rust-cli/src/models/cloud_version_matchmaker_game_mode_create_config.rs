/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerGameModeCreateConfig : Configures the requirements and authentication for the /create endpoint. If this value is not set in the config, the /create endpoint is NOT enabled.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloudVersionMatchmakerGameModeCreateConfig {
    /// Defaults to true when unset.
    #[serde(rename = "enable_private", skip_serializing_if = "Option::is_none")]
    pub enable_private: Option<bool>,
    /// Defaults to false when unset.
    #[serde(rename = "enable_public", skip_serializing_if = "Option::is_none")]
    pub enable_public: Option<bool>,
    /// Sets whether or not the /create endpoint is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "identity_requirement", skip_serializing_if = "Option::is_none")]
    pub identity_requirement: Option<crate::models::CloudVersionMatchmakerGameModeIdentityRequirement>,
    /// **Deprecated**
    #[serde(rename = "max_lobbies_per_identity", skip_serializing_if = "Option::is_none")]
    pub max_lobbies_per_identity: Option<i32>,
    #[serde(rename = "verification", skip_serializing_if = "Option::is_none")]
    pub verification: Option<Box<crate::models::CloudVersionMatchmakerGameModeVerificationConfig>>,
}

impl CloudVersionMatchmakerGameModeCreateConfig {
    /// Configures the requirements and authentication for the /create endpoint. If this value is not set in the config, the /create endpoint is NOT enabled.
    pub fn new(enabled: bool) -> CloudVersionMatchmakerGameModeCreateConfig {
        CloudVersionMatchmakerGameModeCreateConfig {
            enable_private: None,
            enable_public: None,
            enabled,
            identity_requirement: None,
            max_lobbies_per_identity: None,
            verification: None,
        }
    }
}


