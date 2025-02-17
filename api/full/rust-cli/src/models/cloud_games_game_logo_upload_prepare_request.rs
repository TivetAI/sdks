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
pub struct CloudGamesGameLogoUploadPrepareRequest {
    /// Unsigned 64 bit integer.
    #[serde(rename = "content_length")]
    pub content_length: i64,
    /// The MIME type of the game logo.
    #[serde(rename = "mime", skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    /// The path/filename of the game logo.
    #[serde(rename = "path")]
    pub path: String,
}

impl CloudGamesGameLogoUploadPrepareRequest {
    pub fn new(content_length: i64, path: String) -> CloudGamesGameLogoUploadPrepareRequest {
        CloudGamesGameLogoUploadPrepareRequest {
            content_length,
            mime: None,
            path,
        }
    }
}


