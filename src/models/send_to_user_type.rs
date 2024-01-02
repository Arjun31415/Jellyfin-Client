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
pub enum SendToUserType {
    #[serde(rename = "All")]
    All,
    #[serde(rename = "Admins")]
    Admins,
    #[serde(rename = "Custom")]
    Custom,
}

impl ToString for SendToUserType {
    fn to_string(&self) -> String {
        match self {
            Self::All => String::from("All"),
            Self::Admins => String::from("Admins"),
            Self::Custom => String::from("Custom"),
        }
    }
}

impl Default for SendToUserType {
    fn default() -> SendToUserType {
        Self::All
    }
}
