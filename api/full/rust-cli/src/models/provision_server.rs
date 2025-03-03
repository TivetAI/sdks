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
pub struct ProvisionServer {
    #[serde(rename = "datacenter_id")]
    pub datacenter_id: uuid::Uuid,
    #[serde(rename = "pool_type")]
    pub pool_type: crate::models::ProvisionPoolType,
    #[serde(rename = "public_ip", skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    #[serde(rename = "server_id")]
    pub server_id: uuid::Uuid,
    #[serde(rename = "vlan_ip", skip_serializing_if = "Option::is_none")]
    pub vlan_ip: Option<String>,
}

impl ProvisionServer {
    pub fn new(datacenter_id: uuid::Uuid, pool_type: crate::models::ProvisionPoolType, server_id: uuid::Uuid) -> ProvisionServer {
        ProvisionServer {
            datacenter_id,
            pool_type,
            public_ip: None,
            server_id,
            vlan_ip: None,
        }
    }
}


