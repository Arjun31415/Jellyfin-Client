/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// ImageProviderInfo : Class ImageProviderInfo.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageProviderInfo {
    /// Gets the name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Gets the supported image types.
    #[serde(rename = "SupportedImages", skip_serializing_if = "Option::is_none")]
    pub supported_images: Option<Vec<crate::models::ImageType>>,
}

impl ImageProviderInfo {
    /// Class ImageProviderInfo.
    pub fn new() -> ImageProviderInfo {
        ImageProviderInfo {
            name: None,
            supported_images: None,
        }
    }
}
