/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// PingRequestDto : Class PingRequestDto.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PingRequestDto {
    /// Gets or sets the ping time.
    #[serde(rename = "Ping", skip_serializing_if = "Option::is_none")]
    pub ping: Option<i64>,
}

impl PingRequestDto {
    /// Class PingRequestDto.
    pub fn new() -> PingRequestDto {
        PingRequestDto { ping: None }
    }
}
