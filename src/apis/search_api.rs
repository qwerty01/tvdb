/*
 * TVDB API V4
 *
 * Documentation of [TheTVDB](https://thetvdb.com/) API V4. All related information is linked from our [Github repo](https://github.com/thetvdb/v4-api). You might also want to use our [Postman collection] (https://www.getpostman.com/collections/7a9397ce69ff246f74d0) ## Authentication 1. Use the /login endpoint and provide your API key as \"apikey\". If you have a user-supported key, also provide your subscriber PIN as \"pin\". Otherwise completely remove \"pin\" from your call. 2. Executing this call will provide you with a bearer token, which is valid for 1 month. 3. Provide your bearer token for subsequent API calls by clicking Authorize below or including in the header of all direct API calls: `Authorization: Bearer [your-token]`  ## Notes 1. \"score\" is a field across almost all entities.  We generate scores for different types of entities in various ways, so no assumptions should be made about the meaning of this value.  It is simply used to hint at relative popularity for sorting purposes.
 *
 * The version of the OpenAPI document: 4.7.8
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`get_search_results`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSearchResultsError {
    Status401(),
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_search_results_by_remote_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSearchResultsByRemoteIdError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// Our search index includes series, movies, people, and companies. Search is limited to 5k results max.
pub async fn get_search_results(
    configuration: &configuration::Configuration,
    query: Option<&str>,
    q: Option<&str>,
    r#type: Option<&str>,
    year: Option<f32>,
    company: Option<&str>,
    country: Option<&str>,
    director: Option<&str>,
    language: Option<&str>,
    primary_type: Option<&str>,
    network: Option<&str>,
    remote_id: Option<&str>,
    offset: Option<f32>,
    limit: Option<f32>,
) -> Result<crate::models::GetSearchResults200Response, Error<GetSearchResultsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/search", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = query {
        local_var_req_builder =
            local_var_req_builder.query(&[("query", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = q {
        local_var_req_builder = local_var_req_builder.query(&[("q", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = r#type {
        local_var_req_builder =
            local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = year {
        local_var_req_builder =
            local_var_req_builder.query(&[("year", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = company {
        local_var_req_builder =
            local_var_req_builder.query(&[("company", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = country {
        local_var_req_builder =
            local_var_req_builder.query(&[("country", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = director {
        local_var_req_builder =
            local_var_req_builder.query(&[("director", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = language {
        local_var_req_builder =
            local_var_req_builder.query(&[("language", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = primary_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("primaryType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = network {
        local_var_req_builder =
            local_var_req_builder.query(&[("network", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = remote_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("remote_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder =
            local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetSearchResultsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search a series, movie, people, episode, company or season by specific remote id and returns a base record for that entity.
pub async fn get_search_results_by_remote_id(
    configuration: &configuration::Configuration,
    remote_id: &str,
) -> Result<
    crate::models::GetSearchResultsByRemoteId200Response,
    Error<GetSearchResultsByRemoteIdError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/search/remoteid/{remoteId}",
        local_var_configuration.base_path,
        remoteId = crate::apis::urlencode(remote_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetSearchResultsByRemoteIdError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
