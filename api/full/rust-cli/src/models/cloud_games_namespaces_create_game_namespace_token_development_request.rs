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
pub struct CloudGamesNamespacesCreateGameNamespaceTokenDevelopmentRequest {
    /// The hostname used for the token.
    #[serde(rename = "hostname")]
    pub hostname: String,
    /// **Deprecated** A list of docker ports.
    #[serde(rename = "lobby_ports", skip_serializing_if = "Option::is_none")]
    pub lobby_ports: Option<Vec<crate::models::CloudVersionMatchmakerLobbyGroupRuntimeDockerPort>>,
    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<::std::collections::HashMap<String, crate::models::CloudMatchmakerDevelopmentPort>>,
}

impl CloudGamesNamespacesCreateGameNamespaceTokenDevelopmentRequest {
    pub fn new(hostname: String) -> CloudGamesNamespacesCreateGameNamespaceTokenDevelopmentRequest {
        CloudGamesNamespacesCreateGameNamespaceTokenDevelopmentRequest {
            hostname,
            lobby_ports: None,
            ports: None,
        }
    }
}


