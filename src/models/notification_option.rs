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
pub struct NotificationOption {
    #[serde(
        rename = "Type",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<Option<String>>,
    /// Gets or sets user Ids to not monitor (it's opt out).
    #[serde(
        rename = "DisabledMonitorUsers",
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_monitor_users: Option<Vec<String>>,
    /// Gets or sets user Ids to send to (if SendToUserMode == Custom).
    #[serde(rename = "SendToUsers", skip_serializing_if = "Option::is_none")]
    pub send_to_users: Option<Vec<String>>,
    /// Gets or sets a value indicating whether this MediaBrowser.Model.Notifications.NotificationOption is enabled.
    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Gets or sets the disabled services.
    #[serde(rename = "DisabledServices", skip_serializing_if = "Option::is_none")]
    pub disabled_services: Option<Vec<String>>,
    #[serde(rename = "SendToUserMode", skip_serializing_if = "Option::is_none")]
    pub send_to_user_mode: Option<crate::models::SendToUserType>,
}

impl NotificationOption {
    pub fn new() -> NotificationOption {
        NotificationOption {
            r#type: None,
            disabled_monitor_users: None,
            send_to_users: None,
            enabled: None,
            disabled_services: None,
            send_to_user_mode: None,
        }
    }
}
