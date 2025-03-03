/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudLogsLobbySummary : A logs summary for a lobby.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudLogsLobbySummary {
	#[serde(rename = "lobby_id")]
	pub lobby_id: uuid::Uuid,
	#[serde(rename = "namespace_id")]
	pub namespace_id: uuid::Uuid,
	/// A human readable short identifier used to references resources. Different than a `tivet.common#Uuid` because this is intended to be human readable. Different than `tivet.common#DisplayName` because this should not include special characters and be short.
	#[serde(rename = "lobby_group_name_id")]
	pub lobby_group_name_id: String,
	#[serde(rename = "region_id")]
	pub region_id: uuid::Uuid,
	/// RFC3339 timestamp
	#[serde(rename = "create_ts")]
	pub create_ts: String,
	/// RFC3339 timestamp
	#[serde(rename = "start_ts", skip_serializing_if = "Option::is_none")]
	pub start_ts: Option<String>,
	/// RFC3339 timestamp
	#[serde(rename = "ready_ts", skip_serializing_if = "Option::is_none")]
	pub ready_ts: Option<String>,
	#[serde(rename = "status")]
	pub status: Box<crate::models::CloudLogsLobbyStatus>,
}

impl CloudLogsLobbySummary {
	/// A logs summary for a lobby.
	pub fn new(
		lobby_id: uuid::Uuid,
		namespace_id: uuid::Uuid,
		lobby_group_name_id: String,
		region_id: uuid::Uuid,
		create_ts: String,
		status: crate::models::CloudLogsLobbyStatus,
	) -> CloudLogsLobbySummary {
		CloudLogsLobbySummary {
			lobby_id,
			namespace_id,
			lobby_group_name_id,
			region_id,
			create_ts,
			start_ts: None,
			ready_ts: None,
			status: Box::new(status),
		}
	}
}
