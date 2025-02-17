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
pub struct MatchmakerJoinLobbyResponse {
	#[serde(rename = "lobby")]
	pub lobby: Box<crate::models::MatchmakerJoinLobby>,
	#[serde(rename = "ports")]
	pub ports: ::std::collections::HashMap<String, crate::models::MatchmakerJoinPort>,
	#[serde(rename = "player")]
	pub player: Box<crate::models::MatchmakerJoinPlayer>,
}

impl MatchmakerJoinLobbyResponse {
	pub fn new(
		lobby: crate::models::MatchmakerJoinLobby,
		ports: ::std::collections::HashMap<String, crate::models::MatchmakerJoinPort>,
		player: crate::models::MatchmakerJoinPlayer,
	) -> MatchmakerJoinLobbyResponse {
		MatchmakerJoinLobbyResponse {
			lobby: Box::new(lobby),
			ports,
			player: Box::new(player),
		}
	}
}
