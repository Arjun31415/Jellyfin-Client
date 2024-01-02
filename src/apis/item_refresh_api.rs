/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`refresh_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RefreshItemError {
    Status404(crate::models::ProblemDetails),
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

pub async fn refresh_item(
    configuration: &configuration::Configuration,
    item_id: &str,
    metadata_refresh_mode: Option<crate::models::MetadataRefreshMode>,
    image_refresh_mode: Option<crate::models::MetadataRefreshMode>,
    replace_all_metadata: Option<bool>,
    replace_all_images: Option<bool>,
) -> Result<(), Error<RefreshItemError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/Items/{itemId}/Refresh",
        local_var_configuration.base_path,
        itemId = crate::apis::urlencode(item_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = metadata_refresh_mode {
        local_var_req_builder =
            local_var_req_builder.query(&[("metadataRefreshMode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = image_refresh_mode {
        local_var_req_builder =
            local_var_req_builder.query(&[("imageRefreshMode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = replace_all_metadata {
        local_var_req_builder =
            local_var_req_builder.query(&[("replaceAllMetadata", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = replace_all_images {
        local_var_req_builder =
            local_var_req_builder.query(&[("replaceAllImages", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<RefreshItemError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
