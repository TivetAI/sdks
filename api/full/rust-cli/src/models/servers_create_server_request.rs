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
pub struct ServersCreateServerRequest {
    #[serde(rename = "datacenter")]
    pub datacenter: uuid::Uuid,
    #[serde(rename = "lifecycle", skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Box<crate::models::ServersLifecycle>>,
    #[serde(rename = "network")]
    pub network: Box<crate::models::ServersCreateServerNetworkRequest>,
    #[serde(rename = "resources")]
    pub resources: Box<crate::models::ServersResources>,
    #[serde(rename = "runtime")]
    pub runtime: Box<crate::models::ServersCreateServerRuntimeRequest>,
    #[serde(rename = "tags", deserialize_with = "Option::deserialize")]
    pub tags: Option<serde_json::Value>,
}

impl ServersCreateServerRequest {
    pub fn new(datacenter: uuid::Uuid, network: crate::models::ServersCreateServerNetworkRequest, resources: crate::models::ServersResources, runtime: crate::models::ServersCreateServerRuntimeRequest, tags: Option<serde_json::Value>) -> ServersCreateServerRequest {
        ServersCreateServerRequest {
            datacenter,
            lifecycle: None,
            network: Box::new(network),
            resources: Box::new(resources),
            runtime: Box::new(runtime),
            tags,
        }
    }
}


