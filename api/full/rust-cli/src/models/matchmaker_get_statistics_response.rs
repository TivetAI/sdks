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
pub struct MatchmakerGetStatisticsResponse {
    #[serde(rename = "game_modes")]
    pub game_modes: ::std::collections::HashMap<String, crate::models::MatchmakerGameModeStatistics>,
    #[serde(rename = "player_count")]
    pub player_count: i64,
}

impl MatchmakerGetStatisticsResponse {
    pub fn new(game_modes: ::std::collections::HashMap<String, crate::models::MatchmakerGameModeStatistics>, player_count: i64) -> MatchmakerGetStatisticsResponse {
        MatchmakerGetStatisticsResponse {
            game_modes,
            player_count,
        }
    }
}


