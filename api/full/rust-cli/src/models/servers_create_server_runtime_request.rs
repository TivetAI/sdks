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
pub struct ServersCreateServerRuntimeRequest {
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
    #[serde(rename = "build")]
    pub build: uuid::Uuid,
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
}

impl ServersCreateServerRuntimeRequest {
    pub fn new(build: uuid::Uuid) -> ServersCreateServerRuntimeRequest {
        ServersCreateServerRuntimeRequest {
            arguments: None,
            build,
            environment: None,
        }
    }
}


