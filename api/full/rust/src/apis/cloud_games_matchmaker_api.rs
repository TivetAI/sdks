/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`cloud_games_matchmaker_delete_matchmaker_lobby`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudGamesMatchmakerDeleteMatchmakerLobbyError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cloud_games_matchmaker_export_lobby_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudGamesMatchmakerExportLobbyLogsError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cloud_games_matchmaker_export_matchmaker_lobby_history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudGamesMatchmakerExportMatchmakerLobbyHistoryError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cloud_games_matchmaker_get_lobby_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudGamesMatchmakerGetLobbyLogsError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// Deletes a matchmaker lobby, stopping it immediately.
pub async fn cloud_games_matchmaker_delete_matchmaker_lobby(
	configuration: &configuration::Configuration,
	game_id: &str,
	lobby_id: &str,
) -> Result<
	crate::models::CloudGamesDeleteMatchmakerLobbyResponse,
	Error<CloudGamesMatchmakerDeleteMatchmakerLobbyError>,
> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/cloud/games/{game_id}/matchmaker/lobbies/{lobby_id}",
		local_var_configuration.base_path,
		game_id = crate::apis::urlencode(game_id),
		lobby_id = crate::apis::urlencode(lobby_id)
	);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		serde_json::from_str(&local_var_content).map_err(Error::from)
	} else {
		let local_var_entity: Option<CloudGamesMatchmakerDeleteMatchmakerLobbyError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

/// Generates a download URL for logs.
pub async fn cloud_games_matchmaker_export_lobby_logs(
	configuration: &configuration::Configuration,
	game_id: &str,
	lobby_id: &str,
	cloud_games_export_lobby_logs_request: crate::models::CloudGamesExportLobbyLogsRequest,
) -> Result<
	crate::models::CloudGamesExportLobbyLogsResponse,
	Error<CloudGamesMatchmakerExportLobbyLogsError>,
> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/cloud/games/{game_id}/matchmaker/lobbies/{lobby_id}/logs/export",
		local_var_configuration.base_path,
		game_id = crate::apis::urlencode(game_id),
		lobby_id = crate::apis::urlencode(lobby_id)
	);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};
	local_var_req_builder = local_var_req_builder.json(&cloud_games_export_lobby_logs_request);

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		serde_json::from_str(&local_var_content).map_err(Error::from)
	} else {
		let local_var_entity: Option<CloudGamesMatchmakerExportLobbyLogsError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

/// Exports lobby history over a given query time span.
pub async fn cloud_games_matchmaker_export_matchmaker_lobby_history(
	configuration: &configuration::Configuration,
	game_id: &str,
	cloud_games_export_matchmaker_lobby_history_request: crate::models::CloudGamesExportMatchmakerLobbyHistoryRequest,
) -> Result<
	crate::models::CloudGamesExportMatchmakerLobbyHistoryResponse,
	Error<CloudGamesMatchmakerExportMatchmakerLobbyHistoryError>,
> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/cloud/games/{game_id}/matchmaker/lobbies/export-history",
		local_var_configuration.base_path,
		game_id = crate::apis::urlencode(game_id)
	);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};
	local_var_req_builder =
		local_var_req_builder.json(&cloud_games_export_matchmaker_lobby_history_request);

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		serde_json::from_str(&local_var_content).map_err(Error::from)
	} else {
		let local_var_entity: Option<CloudGamesMatchmakerExportMatchmakerLobbyHistoryError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

/// Returns the logs for a given lobby.
pub async fn cloud_games_matchmaker_get_lobby_logs(
	configuration: &configuration::Configuration,
	game_id: &str,
	lobby_id: &str,
	stream: crate::models::CloudGamesLogStream,
	watch_index: Option<&str>,
) -> Result<
	crate::models::CloudGamesGetLobbyLogsResponse,
	Error<CloudGamesMatchmakerGetLobbyLogsError>,
> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/cloud/games/{game_id}/matchmaker/lobbies/{lobby_id}/logs",
		local_var_configuration.base_path,
		game_id = crate::apis::urlencode(game_id),
		lobby_id = crate::apis::urlencode(lobby_id)
	);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

	local_var_req_builder = local_var_req_builder.query(&[("stream", &stream.to_string())]);
	if let Some(ref local_var_str) = watch_index {
		local_var_req_builder =
			local_var_req_builder.query(&[("watch_index", &local_var_str.to_string())]);
	}
	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		serde_json::from_str(&local_var_content).map_err(Error::from)
	} else {
		let local_var_entity: Option<CloudGamesMatchmakerGetLobbyLogsError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}
