/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// PluginStatus : Plugin load status.

/// Plugin load status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PluginStatus {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Restart")]
    Restart,
    #[serde(rename = "Deleted")]
    Deleted,
    #[serde(rename = "Superceded")]
    Superceded,
    #[serde(rename = "Malfunctioned")]
    Malfunctioned,
    #[serde(rename = "NotSupported")]
    NotSupported,
    #[serde(rename = "Disabled")]
    Disabled,
}

impl ToString for PluginStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Active => String::from("Active"),
            Self::Restart => String::from("Restart"),
            Self::Deleted => String::from("Deleted"),
            Self::Superceded => String::from("Superceded"),
            Self::Malfunctioned => String::from("Malfunctioned"),
            Self::NotSupported => String::from("NotSupported"),
            Self::Disabled => String::from("Disabled"),
        }
    }
}

impl Default for PluginStatus {
    fn default() -> PluginStatus {
        Self::Active
    }
}
