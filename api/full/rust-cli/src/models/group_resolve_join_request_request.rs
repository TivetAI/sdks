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
pub struct GroupResolveJoinRequestRequest {
    #[serde(rename = "resolution", skip_serializing_if = "Option::is_none")]
    pub resolution: Option<bool>,
}

impl GroupResolveJoinRequestRequest {
    pub fn new() -> GroupResolveJoinRequestRequest {
        GroupResolveJoinRequestRequest {
            resolution: None,
        }
    }
}


