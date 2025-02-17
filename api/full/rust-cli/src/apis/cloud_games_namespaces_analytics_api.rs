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


/// struct for typed errors of method [`cloud_games_namespaces_analytics_get_analytics_matchmaker_live`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudGamesNamespacesAnalyticsGetAnalyticsMatchmakerLiveError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}


/// Returns live information about all active lobbies for a given namespace.
pub async fn cloud_games_namespaces_analytics_get_analytics_matchmaker_live(configuration: &configuration::Configuration, game_id: &str, namespace_id: &str) -> Result<crate::models::CloudGamesNamespacesGetAnalyticsMatchmakerLiveResponse, Error<CloudGamesNamespacesAnalyticsGetAnalyticsMatchmakerLiveError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud/games/{game_id}/namespaces/{namespace_id}/analytics/matchmaker/live", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), namespace_id=crate::apis::urlencode(namespace_id));
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
        let local_var_entity: Option<CloudGamesNamespacesAnalyticsGetAnalyticsMatchmakerLiveError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

