/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};

/// struct for typed errors of method [`secrets_organization_id_trash_empty_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecretsOrganizationIdTrashEmptyPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`secrets_organization_id_trash_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecretsOrganizationIdTrashGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`secrets_organization_id_trash_restore_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecretsOrganizationIdTrashRestorePostError {
    UnknownValue(serde_json::Value),
}

pub async fn secrets_organization_id_trash_empty_post(
    configuration: &configuration::Configuration,
    organization_id: uuid::Uuid,
    uuid_colon_colon_uuid: Option<Vec<uuid::Uuid>>,
) -> Result<(), Error<SecretsOrganizationIdTrashEmptyPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_organization_id = organization_id;
    let p_uuid_colon_colon_uuid = uuid_colon_colon_uuid;

    let uri_str = format!(
        "{}/secrets/{organizationId}/trash/empty",
        configuration.base_path,
        organizationId = crate::apis::urlencode(p_organization_id.to_string())
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_uuid_colon_colon_uuid);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<SecretsOrganizationIdTrashEmptyPostError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn secrets_organization_id_trash_get(
    configuration: &configuration::Configuration,
    organization_id: uuid::Uuid,
) -> Result<models::SecretWithProjectsListResponseModel, Error<SecretsOrganizationIdTrashGetError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_organization_id = organization_id;

    let uri_str = format!(
        "{}/secrets/{organizationId}/trash",
        configuration.base_path,
        organizationId = crate::apis::urlencode(p_organization_id.to_string())
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SecretWithProjectsListResponseModel`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SecretWithProjectsListResponseModel`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SecretsOrganizationIdTrashGetError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn secrets_organization_id_trash_restore_post(
    configuration: &configuration::Configuration,
    organization_id: uuid::Uuid,
    uuid_colon_colon_uuid: Option<Vec<uuid::Uuid>>,
) -> Result<(), Error<SecretsOrganizationIdTrashRestorePostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_organization_id = organization_id;
    let p_uuid_colon_colon_uuid = uuid_colon_colon_uuid;

    let uri_str = format!(
        "{}/secrets/{organizationId}/trash/restore",
        configuration.base_path,
        organizationId = crate::apis::urlencode(p_organization_id.to_string())
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_uuid_colon_colon_uuid);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<SecretsOrganizationIdTrashRestorePostError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
