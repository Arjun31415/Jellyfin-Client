/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// SpecialViewOptionDto : Special view option dto.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpecialViewOptionDto {
    /// Gets or sets view option name.
    #[serde(
        rename = "Name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    /// Gets or sets view option id.
    #[serde(
        rename = "Id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<Option<String>>,
}

impl SpecialViewOptionDto {
    /// Special view option dto.
    pub fn new() -> SpecialViewOptionDto {
        SpecialViewOptionDto {
            name: None,
            id: None,
        }
    }
}
