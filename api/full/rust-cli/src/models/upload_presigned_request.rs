/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UploadPresignedRequest : A presigned request used to upload files. Upload your file to the given URL via a PUT request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UploadPresignedRequest {
    /// The byte offset for this multipart chunk. Always 0 if not a multipart upload.
    #[serde(rename = "byte_offset")]
    pub byte_offset: i64,
    /// Expected size of this upload.
    #[serde(rename = "content_length")]
    pub content_length: i64,
    /// The name of the file to upload. This is the same as the one given in the upload prepare file.
    #[serde(rename = "path")]
    pub path: String,
    /// The URL of the presigned request for which to upload your file to.
    #[serde(rename = "url")]
    pub url: String,
}

impl UploadPresignedRequest {
    /// A presigned request used to upload files. Upload your file to the given URL via a PUT request.
    pub fn new(byte_offset: i64, content_length: i64, path: String, url: String) -> UploadPresignedRequest {
        UploadPresignedRequest {
            byte_offset,
            content_length,
            path,
            url,
        }
    }
}


