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
pub enum LiveTvServiceStatus {
    #[serde(rename = "Ok")]
    Ok,
    #[serde(rename = "Unavailable")]
    Unavailable,
}

impl ToString for LiveTvServiceStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Ok => String::from("Ok"),
            Self::Unavailable => String::from("Unavailable"),
        }
    }
}

impl Default for LiveTvServiceStatus {
    fn default() -> LiveTvServiceStatus {
        Self::Ok
    }
}
