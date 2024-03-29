/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotificationLevel {
    #[serde(rename = "Normal")]
    Normal,
    #[serde(rename = "Warning")]
    Warning,
    #[serde(rename = "Error")]
    Error,
}

impl ToString for NotificationLevel {
    fn to_string(&self) -> String {
        match self {
            Self::Normal => String::from("Normal"),
            Self::Warning => String::from("Warning"),
            Self::Error => String::from("Error"),
        }
    }
}

impl Default for NotificationLevel {
    fn default() -> NotificationLevel {
        Self::Normal
    }
}
