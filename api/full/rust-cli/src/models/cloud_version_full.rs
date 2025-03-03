/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionFull : A full version.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloudVersionFull {
    #[serde(rename = "config")]
    pub config: Box<crate::models::CloudVersionConfig>,
    /// RFC3339 timestamp
    #[serde(rename = "create_ts")]
    pub create_ts: String,
    /// Represent a resource's readable display name.
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "version_id")]
    pub version_id: uuid::Uuid,
}

impl CloudVersionFull {
    /// A full version.
    pub fn new(config: crate::models::CloudVersionConfig, create_ts: String, display_name: String, version_id: uuid::Uuid) -> CloudVersionFull {
        CloudVersionFull {
            config: Box::new(config),
            create_ts,
            display_name,
            version_id,
        }
    }
}


