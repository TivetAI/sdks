/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UploadPrepareFile : A file being prepared to upload.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UploadPrepareFile {
    /// Unsigned 64 bit integer.
    #[serde(rename = "content_length")]
    pub content_length: i64,
    /// The MIME type of the file.
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The path/filename of the file.
    #[serde(rename = "path")]
    pub path: String,
}

impl UploadPrepareFile {
    /// A file being prepared to upload.
    pub fn new(content_length: i64, path: String) -> UploadPrepareFile {
        UploadPrepareFile {
            content_length,
            content_type: None,
            path,
        }
    }
}


