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
pub struct CloudGamesValidateGameVersionResponse {
    /// A list of validation errors.
    #[serde(rename = "errors")]
    pub errors: Vec<crate::models::ValidationError>,
}

impl CloudGamesValidateGameVersionResponse {
    pub fn new(errors: Vec<crate::models::ValidationError>) -> CloudGamesValidateGameVersionResponse {
        CloudGamesValidateGameVersionResponse {
            errors,
        }
    }
}


