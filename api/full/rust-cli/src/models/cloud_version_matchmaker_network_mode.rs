/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerNetworkMode : Configures how the container's network is isolated from the host. `bridge` (default) networking isolates the container's network from the host & other containers. `host` networking removes isolation between the container and the host. Only available in Tivet Open Source & Enterprise. Read more about bridge vs host networking [here](https://tivet.gg/docs/dynamic-servers/concepts/host-bridge-networking).

/// Configures how the container's network is isolated from the host. `bridge` (default) networking isolates the container's network from the host & other containers. `host` networking removes isolation between the container and the host. Only available in Tivet Open Source & Enterprise. Read more about bridge vs host networking [here](https://tivet.gg/docs/dynamic-servers/concepts/host-bridge-networking).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CloudVersionMatchmakerNetworkMode {
    #[serde(rename = "bridge")]
    Bridge,
    #[serde(rename = "host")]
    Host,

}

impl ToString for CloudVersionMatchmakerNetworkMode {
    fn to_string(&self) -> String {
        match self {
            Self::Bridge => String::from("bridge"),
            Self::Host => String::from("host"),
        }
    }
}

impl Default for CloudVersionMatchmakerNetworkMode {
    fn default() -> CloudVersionMatchmakerNetworkMode {
        Self::Bridge
    }
}




