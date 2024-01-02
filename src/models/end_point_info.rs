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
pub struct EndPointInfo {
    #[serde(rename = "IsLocal", skip_serializing_if = "Option::is_none")]
    pub is_local: Option<bool>,
    #[serde(rename = "IsInNetwork", skip_serializing_if = "Option::is_none")]
    pub is_in_network: Option<bool>,
}

impl EndPointInfo {
    pub fn new() -> EndPointInfo {
        EndPointInfo {
            is_local: None,
            is_in_network: None,
        }
    }
}
