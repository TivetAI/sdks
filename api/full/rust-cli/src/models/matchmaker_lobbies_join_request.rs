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
pub struct MatchmakerLobbiesJoinRequest {
    #[serde(rename = "captcha", skip_serializing_if = "Option::is_none")]
    pub captcha: Option<Box<crate::models::CaptchaConfig>>,
    #[serde(rename = "lobby_id")]
    pub lobby_id: String,
    #[serde(rename = "verification_data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub verification_data: Option<Option<serde_json::Value>>,
}

impl MatchmakerLobbiesJoinRequest {
    pub fn new(lobby_id: String) -> MatchmakerLobbiesJoinRequest {
        MatchmakerLobbiesJoinRequest {
            captcha: None,
            lobby_id,
            verification_data: None,
        }
    }
}


