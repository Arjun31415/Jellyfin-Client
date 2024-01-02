/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// SearchHintResult : Class SearchHintResult.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchHintResult {
    /// Gets the search hints.
    #[serde(rename = "SearchHints", skip_serializing_if = "Option::is_none")]
    pub search_hints: Option<Vec<crate::models::SearchHint>>,
    /// Gets the total record count.
    #[serde(rename = "TotalRecordCount", skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
}

impl SearchHintResult {
    /// Class SearchHintResult.
    pub fn new() -> SearchHintResult {
        SearchHintResult {
            search_hints: None,
            total_record_count: None,
        }
    }
}
