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
pub struct CloudGamesGameBannerUploadPrepareResponse {
    #[serde(rename = "presigned_request")]
    pub presigned_request: Box<crate::models::UploadPresignedRequest>,
    #[serde(rename = "upload_id")]
    pub upload_id: uuid::Uuid,
}

impl CloudGamesGameBannerUploadPrepareResponse {
    pub fn new(presigned_request: crate::models::UploadPresignedRequest, upload_id: uuid::Uuid) -> CloudGamesGameBannerUploadPrepareResponse {
        CloudGamesGameBannerUploadPrepareResponse {
            presigned_request: Box::new(presigned_request),
            upload_id,
        }
    }
}


