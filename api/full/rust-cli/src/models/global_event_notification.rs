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
pub struct GlobalEventNotification {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "thumbnail_url")]
    pub thumbnail_url: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "url")]
    pub url: String,
}

impl GlobalEventNotification {
    pub fn new(description: String, thumbnail_url: String, title: String, url: String) -> GlobalEventNotification {
        GlobalEventNotification {
            description,
            thumbnail_url,
            title,
            url,
        }
    }
}


