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

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`matchmaker_lobbies_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatchmakerLobbiesCreateError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`matchmaker_lobbies_find`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatchmakerLobbiesFindError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`matchmaker_lobbies_get_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatchmakerLobbiesGetStateError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`matchmaker_lobbies_join`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatchmakerLobbiesJoinError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`matchmaker_lobbies_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatchmakerLobbiesListError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`matchmaker_lobbies_ready`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatchmakerLobbiesReadyError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`matchmaker_lobbies_set_closed`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatchmakerLobbiesSetClosedError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`matchmaker_lobbies_set_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatchmakerLobbiesSetStateError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}


/// Creates a custom lobby.  When [tokenless authentication](/docs/general/concepts/tokenless-authentication/web) is enabled in your game namespace, this endpoint does not require a token to authenticate. Otherwise, a [development namespace token](/docs/general/concepts/token-types#namespace-development) can be used for mock responses and a [public namespace token](/docs/general/concepts/token-types#namespace-public) can be used for general authentication.
pub async fn matchmaker_lobbies_create(configuration: &configuration::Configuration, matchmaker_lobbies_create_request: crate::models::MatchmakerLobbiesCreateRequest) -> Result<crate::models::MatchmakerCreateLobbyResponse, Error<MatchmakerLobbiesCreateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/matchmaker/lobbies/create", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&matchmaker_lobbies_create_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MatchmakerLobbiesCreateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Finds a lobby based on the given criteria. If a lobby is not found and `prevent_auto_create_lobby` is `false`, a new lobby will be created.  When [tokenless authentication](/docs/general/concepts/tokenless-authentication/web) is enabled in your game namespace, this endpoint does not require a token to authenticate. Otherwise, a [development namespace token](/docs/general/concepts/token-types#namespace-development) can be used for mock responses and a [public namespace token](/docs/general/concepts/token-types#namespace-public) can be used for general authentication.
pub async fn matchmaker_lobbies_find(configuration: &configuration::Configuration, matchmaker_lobbies_find_request: crate::models::MatchmakerLobbiesFindRequest, origin: Option<&str>) -> Result<crate::models::MatchmakerFindLobbyResponse, Error<MatchmakerLobbiesFindError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/matchmaker/lobbies/find", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = origin {
        local_var_req_builder = local_var_req_builder.header("origin", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&matchmaker_lobbies_find_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MatchmakerLobbiesFindError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the state of any lobby.  This endpoint requires a [lobby token](/docs/general/concepts/token-types#matchmaker-lobby) for authentication, or a [development namespace token](/docs/general/concepts/token-types#namespace-development) for mock responses. When running on Tivet servers, you can access the given lobby token from the [`TIVET_TOKEN`](/docs/matchmaker/concepts/lobby-env) environment variable.
pub async fn matchmaker_lobbies_get_state(configuration: &configuration::Configuration, lobby_id: &str) -> Result<serde_json::Value, Error<MatchmakerLobbiesGetStateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/matchmaker/lobbies/{lobby_id}/state", local_var_configuration.base_path, lobby_id=crate::apis::urlencode(lobby_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<MatchmakerLobbiesGetStateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Joins a specific lobby. This request will use the direct player count configured for the lobby group.  When [tokenless authentication](/docs/general/concepts/tokenless-authentication/web) is enabled in your game namespace, this endpoint does not require a token to authenticate. Otherwise, a [development namespace token](/docs/general/concepts/token-types#namespace-development) can be used for mock responses and a [public namespace token](/docs/general/concepts/token-types#namespace-public) can be used for general authentication.
pub async fn matchmaker_lobbies_join(configuration: &configuration::Configuration, matchmaker_lobbies_join_request: crate::models::MatchmakerLobbiesJoinRequest) -> Result<crate::models::MatchmakerJoinLobbyResponse, Error<MatchmakerLobbiesJoinError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/matchmaker/lobbies/join", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&matchmaker_lobbies_join_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MatchmakerLobbiesJoinError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists all open lobbies.  When [tokenless authentication](/docs/general/concepts/tokenless-authentication/web) is enabled in your game namespace, this endpoint does not require a token to authenticate. Otherwise, a [development namespace token](/docs/general/concepts/token-types#namespace-development) can be used for mock responses and a [public namespace token](/docs/general/concepts/token-types#namespace-public) can be used for general authentication.
pub async fn matchmaker_lobbies_list(configuration: &configuration::Configuration, include_state: Option<bool>) -> Result<crate::models::MatchmakerListLobbiesResponse, Error<MatchmakerLobbiesListError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/matchmaker/lobbies/list", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = include_state {
        local_var_req_builder = local_var_req_builder.query(&[("include_state", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<MatchmakerLobbiesListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Marks the current lobby as ready to accept connections. Players will not be able to connect to this lobby until the lobby is flagged as ready. This endpoint requires a [lobby token](/docs/general/concepts/token-types#matchmaker-lobby) for authentication, or a [development namespace token](/docs/general/concepts/token-types#namespace-development) for mock responses. When running on Tivet servers, you can access the given lobby token from the [`TIVET_TOKEN`](/docs/matchmaker/concepts/lobby-env) environment variable.
pub async fn matchmaker_lobbies_ready(configuration: &configuration::Configuration, ) -> Result<(), Error<MatchmakerLobbiesReadyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/matchmaker/lobbies/ready", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<MatchmakerLobbiesReadyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// If `is_closed` is `true`, the matchmaker will no longer route players to the lobby. Players can still join using the /join endpoint (this can be disabled by the developer by rejecting all new connections after setting the lobby to closed). Does not shutdown the lobby.  This endpoint requires a [lobby token](/docs/general/concepts/token-types#matchmaker-lobby) for authentication, or a [development namespace token](/docs/general/concepts/token-types#namespace-development) for mock responses. When running on Tivet servers, you can access the given lobby token from the [`TIVET_TOKEN`](/docs/matchmaker/concepts/lobby-env) environment variable.
pub async fn matchmaker_lobbies_set_closed(configuration: &configuration::Configuration, matchmaker_lobbies_set_closed_request: crate::models::MatchmakerLobbiesSetClosedRequest) -> Result<(), Error<MatchmakerLobbiesSetClosedError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/matchmaker/lobbies/closed", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&matchmaker_lobbies_set_closed_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<MatchmakerLobbiesSetClosedError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sets the state JSON of the current lobby.  This endpoint requires a [lobby token](/docs/general/concepts/token-types#matchmaker-lobby) for authentication, or a [development namespace token](/docs/general/concepts/token-types#namespace-development) for mock responses. When running on Tivet servers, you can access the given lobby token from the [`TIVET_TOKEN`](/docs/matchmaker/concepts/lobby-env) environment variable.
pub async fn matchmaker_lobbies_set_state(configuration: &configuration::Configuration, body: Option<serde_json::Value>) -> Result<(), Error<MatchmakerLobbiesSetStateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/matchmaker/lobbies/state", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<MatchmakerLobbiesSetStateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

