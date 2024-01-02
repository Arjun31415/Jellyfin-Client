/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// CountryInfo : Class CountryInfo.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CountryInfo {
    /// Gets or sets the name.
    #[serde(
        rename = "Name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    /// Gets or sets the display name.
    #[serde(
        rename = "DisplayName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<Option<String>>,
    /// Gets or sets the name of the two letter ISO region.
    #[serde(
        rename = "TwoLetterISORegionName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub two_letter_iso_region_name: Option<Option<String>>,
    /// Gets or sets the name of the three letter ISO region.
    #[serde(
        rename = "ThreeLetterISORegionName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub three_letter_iso_region_name: Option<Option<String>>,
}

impl CountryInfo {
    /// Class CountryInfo.
    pub fn new() -> CountryInfo {
        CountryInfo {
            name: None,
            display_name: None,
            two_letter_iso_region_name: None,
            three_letter_iso_region_name: None,
        }
    }
}
