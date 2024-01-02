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
pub struct HttpHeaderInfo {
    #[serde(
        rename = "Name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    #[serde(
        rename = "Value",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<Option<String>>,
    #[serde(rename = "Match", skip_serializing_if = "Option::is_none")]
    pub r#match: Option<crate::models::HeaderMatchType>,
}

impl HttpHeaderInfo {
    pub fn new() -> HttpHeaderInfo {
        HttpHeaderInfo {
            name: None,
            value: None,
            r#match: None,
        }
    }
}
