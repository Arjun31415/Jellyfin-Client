/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// BaseItemPersonImageBlurHashes : Gets or sets the primary image blurhash.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseItemPersonImageBlurHashes {
    #[serde(rename = "Primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Art", skip_serializing_if = "Option::is_none")]
    pub art: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Backdrop", skip_serializing_if = "Option::is_none")]
    pub backdrop: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Banner", skip_serializing_if = "Option::is_none")]
    pub banner: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Thumb", skip_serializing_if = "Option::is_none")]
    pub thumb: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Disc", skip_serializing_if = "Option::is_none")]
    pub disc: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Box", skip_serializing_if = "Option::is_none")]
    pub r#box: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Screenshot", skip_serializing_if = "Option::is_none")]
    pub screenshot: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Menu", skip_serializing_if = "Option::is_none")]
    pub menu: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Chapter", skip_serializing_if = "Option::is_none")]
    pub chapter: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "BoxRear", skip_serializing_if = "Option::is_none")]
    pub box_rear: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Profile", skip_serializing_if = "Option::is_none")]
    pub profile: Option<::std::collections::HashMap<String, String>>,
}

impl BaseItemPersonImageBlurHashes {
    /// Gets or sets the primary image blurhash.
    pub fn new() -> BaseItemPersonImageBlurHashes {
        BaseItemPersonImageBlurHashes {
            primary: None,
            art: None,
            backdrop: None,
            banner: None,
            logo: None,
            thumb: None,
            disc: None,
            r#box: None,
            screenshot: None,
            menu: None,
            chapter: None,
            box_rear: None,
            profile: None,
        }
    }
}