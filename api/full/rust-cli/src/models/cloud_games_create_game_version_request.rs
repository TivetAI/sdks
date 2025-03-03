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
pub struct CloudGamesCreateGameVersionRequest {
    #[serde(rename = "config")]
    pub config: Box<crate::models::CloudVersionConfig>,
    /// Represent a resource's readable display name.
    #[serde(rename = "display_name")]
    pub display_name: String,
}

impl CloudGamesCreateGameVersionRequest {
    pub fn new(config: crate::models::CloudVersionConfig, display_name: String) -> CloudGamesCreateGameVersionRequest {
        CloudGamesCreateGameVersionRequest {
            config: Box::new(config),
            display_name,
        }
    }
}


