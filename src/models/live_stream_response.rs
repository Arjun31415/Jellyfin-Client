/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveStreamResponse {
    #[serde(rename = "MediaSource", skip_serializing_if = "Option::is_none")]
    pub media_source: Option<Box<crate::models::MediaSourceInfo>>,
}

impl LiveStreamResponse {
    pub fn new() -> LiveStreamResponse {
        LiveStreamResponse { media_source: None }
    }
}
