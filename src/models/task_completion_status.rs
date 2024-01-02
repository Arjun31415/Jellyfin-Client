/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// TaskCompletionStatus : Enum TaskCompletionStatus.

/// Enum TaskCompletionStatus.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaskCompletionStatus {
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Aborted")]
    Aborted,
}

impl ToString for TaskCompletionStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Completed => String::from("Completed"),
            Self::Failed => String::from("Failed"),
            Self::Cancelled => String::from("Cancelled"),
            Self::Aborted => String::from("Aborted"),
        }
    }
}

impl Default for TaskCompletionStatus {
    fn default() -> TaskCompletionStatus {
        Self::Completed
    }
}
