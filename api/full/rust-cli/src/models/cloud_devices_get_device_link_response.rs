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
pub struct CloudDevicesGetDeviceLinkResponse {
    #[serde(rename = "cloud_token", skip_serializing_if = "Option::is_none")]
    pub cloud_token: Option<String>,
    #[serde(rename = "watch")]
    pub watch: Box<crate::models::WatchResponse>,
}

impl CloudDevicesGetDeviceLinkResponse {
    pub fn new(watch: crate::models::WatchResponse) -> CloudDevicesGetDeviceLinkResponse {
        CloudDevicesGetDeviceLinkResponse {
            cloud_token: None,
            watch: Box::new(watch),
        }
    }
}


