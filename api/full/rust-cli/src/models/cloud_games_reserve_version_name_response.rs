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
pub struct CloudGamesReserveVersionNameResponse {
    /// Represent a resource's readable display name.
    #[serde(rename = "version_display_name")]
    pub version_display_name: String,
}

impl CloudGamesReserveVersionNameResponse {
    pub fn new(version_display_name: String) -> CloudGamesReserveVersionNameResponse {
        CloudGamesReserveVersionNameResponse {
            version_display_name,
        }
    }
}


