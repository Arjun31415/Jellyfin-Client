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

/// struct for typed errors of method [`get_universal_audio_stream`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUniversalAudioStreamError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`head_universal_audio_stream`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeadUniversalAudioStreamError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

pub async fn get_universal_audio_stream(
    configuration: &configuration::Configuration,
    item_id: &str,
    container: Option<Vec<String>>,
    media_source_id: Option<&str>,
    device_id: Option<&str>,
    user_id: Option<&str>,
    audio_codec: Option<&str>,
    max_audio_channels: Option<i32>,
    transcoding_audio_channels: Option<i32>,
    max_streaming_bitrate: Option<i32>,
    audio_bit_rate: Option<i32>,
    start_time_ticks: Option<i64>,
    transcoding_container: Option<&str>,
    transcoding_protocol: Option<&str>,
    max_audio_sample_rate: Option<i32>,
    max_audio_bit_depth: Option<i32>,
    enable_remote_media: Option<bool>,
    break_on_non_key_frames: Option<bool>,
    enable_redirection: Option<bool>,
) -> Result<std::path::PathBuf, Error<GetUniversalAudioStreamError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/Audio/{itemId}/universal",
        local_var_configuration.base_path,
        itemId = crate::apis::urlencode(item_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = container {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("container".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "container",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = media_source_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("mediaSourceId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = device_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("deviceId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = audio_codec {
        local_var_req_builder =
            local_var_req_builder.query(&[("audioCodec", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_audio_channels {
        local_var_req_builder =
            local_var_req_builder.query(&[("maxAudioChannels", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = transcoding_audio_channels {
        local_var_req_builder = local_var_req_builder
            .query(&[("transcodingAudioChannels", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_streaming_bitrate {
        local_var_req_builder =
            local_var_req_builder.query(&[("maxStreamingBitrate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = audio_bit_rate {
        local_var_req_builder =
            local_var_req_builder.query(&[("audioBitRate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_time_ticks {
        local_var_req_builder =
            local_var_req_builder.query(&[("startTimeTicks", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = transcoding_container {
        local_var_req_builder =
            local_var_req_builder.query(&[("transcodingContainer", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = transcoding_protocol {
        local_var_req_builder =
            local_var_req_builder.query(&[("transcodingProtocol", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_audio_sample_rate {
        local_var_req_builder =
            local_var_req_builder.query(&[("maxAudioSampleRate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_audio_bit_depth {
        local_var_req_builder =
            local_var_req_builder.query(&[("maxAudioBitDepth", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = enable_remote_media {
        local_var_req_builder =
            local_var_req_builder.query(&[("enableRemoteMedia", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = break_on_non_key_frames {
        local_var_req_builder =
            local_var_req_builder.query(&[("breakOnNonKeyFrames", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = enable_redirection {
        local_var_req_builder =
            local_var_req_builder.query(&[("enableRedirection", &local_var_str.to_string())]);
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
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUniversalAudioStreamError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn head_universal_audio_stream(
    configuration: &configuration::Configuration,
    item_id: &str,
    container: Option<Vec<String>>,
    media_source_id: Option<&str>,
    device_id: Option<&str>,
    user_id: Option<&str>,
    audio_codec: Option<&str>,
    max_audio_channels: Option<i32>,
    transcoding_audio_channels: Option<i32>,
    max_streaming_bitrate: Option<i32>,
    audio_bit_rate: Option<i32>,
    start_time_ticks: Option<i64>,
    transcoding_container: Option<&str>,
    transcoding_protocol: Option<&str>,
    max_audio_sample_rate: Option<i32>,
    max_audio_bit_depth: Option<i32>,
    enable_remote_media: Option<bool>,
    break_on_non_key_frames: Option<bool>,
    enable_redirection: Option<bool>,
) -> Result<std::path::PathBuf, Error<HeadUniversalAudioStreamError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/Audio/{itemId}/universal",
        local_var_configuration.base_path,
        itemId = crate::apis::urlencode(item_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::HEAD, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = container {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("container".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "container",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = media_source_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("mediaSourceId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = device_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("deviceId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = audio_codec {
        local_var_req_builder =
            local_var_req_builder.query(&[("audioCodec", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_audio_channels {
        local_var_req_builder =
            local_var_req_builder.query(&[("maxAudioChannels", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = transcoding_audio_channels {
        local_var_req_builder = local_var_req_builder
            .query(&[("transcodingAudioChannels", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_streaming_bitrate {
        local_var_req_builder =
            local_var_req_builder.query(&[("maxStreamingBitrate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = audio_bit_rate {
        local_var_req_builder =
            local_var_req_builder.query(&[("audioBitRate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_time_ticks {
        local_var_req_builder =
            local_var_req_builder.query(&[("startTimeTicks", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = transcoding_container {
        local_var_req_builder =
            local_var_req_builder.query(&[("transcodingContainer", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = transcoding_protocol {
        local_var_req_builder =
            local_var_req_builder.query(&[("transcodingProtocol", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_audio_sample_rate {
        local_var_req_builder =
            local_var_req_builder.query(&[("maxAudioSampleRate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_audio_bit_depth {
        local_var_req_builder =
            local_var_req_builder.query(&[("maxAudioBitDepth", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = enable_remote_media {
        local_var_req_builder =
            local_var_req_builder.query(&[("enableRemoteMedia", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = break_on_non_key_frames {
        local_var_req_builder =
            local_var_req_builder.query(&[("breakOnNonKeyFrames", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = enable_redirection {
        local_var_req_builder =
            local_var_req_builder.query(&[("enableRedirection", &local_var_str.to_string())]);
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
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<HeadUniversalAudioStreamError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
