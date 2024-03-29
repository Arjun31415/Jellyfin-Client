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
pub enum KeepUntil {
    #[serde(rename = "UntilDeleted")]
    UntilDeleted,
    #[serde(rename = "UntilSpaceNeeded")]
    UntilSpaceNeeded,
    #[serde(rename = "UntilWatched")]
    UntilWatched,
    #[serde(rename = "UntilDate")]
    UntilDate,
}

impl ToString for KeepUntil {
    fn to_string(&self) -> String {
        match self {
            Self::UntilDeleted => String::from("UntilDeleted"),
            Self::UntilSpaceNeeded => String::from("UntilSpaceNeeded"),
            Self::UntilWatched => String::from("UntilWatched"),
            Self::UntilDate => String::from("UntilDate"),
        }
    }
}

impl Default for KeepUntil {
    fn default() -> KeepUntil {
        Self::UntilDeleted
    }
}
