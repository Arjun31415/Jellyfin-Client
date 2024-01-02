/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChapterInfo : Class ChapterInfo.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChapterInfo {
    /// Gets or sets the start position ticks.
    #[serde(rename = "StartPositionTicks", skip_serializing_if = "Option::is_none")]
    pub start_position_ticks: Option<i64>,
    /// Gets or sets the name.
    #[serde(
        rename = "Name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    /// Gets or sets the image path.
    #[serde(
        rename = "ImagePath",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_path: Option<Option<String>>,
    #[serde(rename = "ImageDateModified", skip_serializing_if = "Option::is_none")]
    pub image_date_modified: Option<String>,
    #[serde(
        rename = "ImageTag",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_tag: Option<Option<String>>,
}

impl ChapterInfo {
    /// Class ChapterInfo.
    pub fn new() -> ChapterInfo {
        ChapterInfo {
            start_position_ticks: None,
            name: None,
            image_path: None,
            image_date_modified: None,
            image_tag: None,
        }
    }
}
