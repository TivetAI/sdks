/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerLobbyGroupRuntimeDocker : **Deprecated: use GameMode instead** A game mode runtime running through Docker.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloudVersionMatchmakerLobbyGroupRuntimeDocker {
    #[serde(rename = "args")]
    pub args: Vec<String>,
    #[serde(rename = "build_id", skip_serializing_if = "Option::is_none")]
    pub build_id: Option<uuid::Uuid>,
    #[serde(rename = "env_vars")]
    pub env_vars: Vec<crate::models::CloudVersionMatchmakerLobbyGroupRuntimeDockerEnvVar>,
    #[serde(rename = "network_mode", skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<crate::models::CloudVersionMatchmakerNetworkMode>,
    #[serde(rename = "ports")]
    pub ports: Vec<crate::models::CloudVersionMatchmakerLobbyGroupRuntimeDockerPort>,
}

impl CloudVersionMatchmakerLobbyGroupRuntimeDocker {
    /// **Deprecated: use GameMode instead** A game mode runtime running through Docker.
    pub fn new(args: Vec<String>, env_vars: Vec<crate::models::CloudVersionMatchmakerLobbyGroupRuntimeDockerEnvVar>, ports: Vec<crate::models::CloudVersionMatchmakerLobbyGroupRuntimeDockerPort>) -> CloudVersionMatchmakerLobbyGroupRuntimeDocker {
        CloudVersionMatchmakerLobbyGroupRuntimeDocker {
            args,
            build_id: None,
            env_vars,
            network_mode: None,
            ports,
        }
    }
}


