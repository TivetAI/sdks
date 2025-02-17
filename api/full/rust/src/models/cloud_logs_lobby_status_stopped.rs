/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudLogsLobbyStatusStopped : The status of a stopped lobby.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudLogsLobbyStatusStopped {
	/// RFC3339 timestamp
	#[serde(rename = "stop_ts")]
	pub stop_ts: String,
	/// Whether or not the lobby failed or stopped successfully.
	#[serde(rename = "failed")]
	pub failed: bool,
	/// The exit code returned by the lobby's main process when stopped.
	#[serde(rename = "exit_code")]
	pub exit_code: i32,
}

impl CloudLogsLobbyStatusStopped {
	/// The status of a stopped lobby.
	pub fn new(stop_ts: String, failed: bool, exit_code: i32) -> CloudLogsLobbyStatusStopped {
		CloudLogsLobbyStatusStopped {
			stop_ts,
			failed,
			exit_code,
		}
	}
}
