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
pub struct TimerEventInfo {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "ProgramId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub program_id: Option<Option<uuid::Uuid>>,
}

impl TimerEventInfo {
    pub fn new() -> TimerEventInfo {
        TimerEventInfo {
            id: None,
            program_id: None,
        }
    }
}
