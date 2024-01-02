/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// UpdateUserEasyPassword : The update user easy password request body.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateUserEasyPassword {
    /// Gets or sets the new sha1-hashed password.
    #[serde(
        rename = "NewPassword",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub new_password: Option<Option<String>>,
    /// Gets or sets the new password.
    #[serde(
        rename = "NewPw",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub new_pw: Option<Option<String>>,
    /// Gets or sets a value indicating whether to reset the password.
    #[serde(rename = "ResetPassword", skip_serializing_if = "Option::is_none")]
    pub reset_password: Option<bool>,
}

impl UpdateUserEasyPassword {
    /// The update user easy password request body.
    pub fn new() -> UpdateUserEasyPassword {
        UpdateUserEasyPassword {
            new_password: None,
            new_pw: None,
            reset_password: None,
        }
    }
}
