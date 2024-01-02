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
pub struct PlaystateRequest {
    #[serde(rename = "Command", skip_serializing_if = "Option::is_none")]
    pub command: Option<crate::models::PlaystateCommand>,
    #[serde(
        rename = "SeekPositionTicks",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub seek_position_ticks: Option<Option<i64>>,
    /// Gets or sets the controlling user identifier.
    #[serde(
        rename = "ControllingUserId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub controlling_user_id: Option<Option<String>>,
}

impl PlaystateRequest {
    pub fn new() -> PlaystateRequest {
        PlaystateRequest {
            command: None,
            seek_position_ticks: None,
            controlling_user_id: None,
        }
    }
}
