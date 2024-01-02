/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// UserDto : Class UserDto.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDto {
    /// Gets or sets the name.
    #[serde(
        rename = "Name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    /// Gets or sets the server identifier.
    #[serde(
        rename = "ServerId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub server_id: Option<Option<String>>,
    /// Gets or sets the name of the server.  This is not used by the server and is for client-side usage only.
    #[serde(
        rename = "ServerName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub server_name: Option<Option<String>>,
    /// Gets or sets the id.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Gets or sets the primary image tag.
    #[serde(
        rename = "PrimaryImageTag",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_image_tag: Option<Option<String>>,
    /// Gets or sets a value indicating whether this instance has password.
    #[serde(rename = "HasPassword", skip_serializing_if = "Option::is_none")]
    pub has_password: Option<bool>,
    /// Gets or sets a value indicating whether this instance has configured password.
    #[serde(
        rename = "HasConfiguredPassword",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_configured_password: Option<bool>,
    /// Gets or sets a value indicating whether this instance has configured easy password.
    #[serde(
        rename = "HasConfiguredEasyPassword",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_configured_easy_password: Option<bool>,
    /// Gets or sets whether async login is enabled or not.
    #[serde(
        rename = "EnableAutoLogin",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_auto_login: Option<Option<bool>>,
    /// Gets or sets the last login date.
    #[serde(
        rename = "LastLoginDate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_login_date: Option<Option<String>>,
    /// Gets or sets the last activity date.
    #[serde(
        rename = "LastActivityDate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_activity_date: Option<Option<String>>,
    #[serde(
        rename = "Configuration",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration: Option<Option<Box<crate::models::UserDtoConfiguration>>>,
    #[serde(
        rename = "Policy",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub policy: Option<Option<Box<crate::models::UserDtoPolicy>>>,
    /// Gets or sets the primary image aspect ratio.
    #[serde(
        rename = "PrimaryImageAspectRatio",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_image_aspect_ratio: Option<Option<f64>>,
}

impl UserDto {
    /// Class UserDto.
    pub fn new() -> UserDto {
        UserDto {
            name: None,
            server_id: None,
            server_name: None,
            id: None,
            primary_image_tag: None,
            has_password: None,
            has_configured_password: None,
            has_configured_easy_password: None,
            enable_auto_login: None,
            last_login_date: None,
            last_activity_date: None,
            configuration: None,
            policy: None,
            primary_image_aspect_ratio: None,
        }
    }
}
