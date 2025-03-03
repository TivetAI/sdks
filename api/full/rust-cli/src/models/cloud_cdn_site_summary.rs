/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudCdnSiteSummary : A CDN site summary.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudCdnSiteSummary {
    /// Whether or not this site has completely been uploaded.
    #[serde(rename = "complete")]
    pub complete: bool,
    /// Unsigned 64 bit integer.
    #[serde(rename = "content_length")]
    pub content_length: i64,
    /// RFC3339 timestamp
    #[serde(rename = "create_ts")]
    pub create_ts: String,
    /// Represent a resource's readable display name.
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "site_id")]
    pub site_id: uuid::Uuid,
    #[serde(rename = "upload_id")]
    pub upload_id: uuid::Uuid,
}

impl CloudCdnSiteSummary {
    /// A CDN site summary.
    pub fn new(complete: bool, content_length: i64, create_ts: String, display_name: String, site_id: uuid::Uuid, upload_id: uuid::Uuid) -> CloudCdnSiteSummary {
        CloudCdnSiteSummary {
            complete,
            content_length,
            create_ts,
            display_name,
            site_id,
            upload_id,
        }
    }
}


