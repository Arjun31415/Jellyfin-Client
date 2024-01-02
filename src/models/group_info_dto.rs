/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// GroupInfoDto : Class GroupInfoDto.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupInfoDto {
    /// Gets the group identifier.
    #[serde(rename = "GroupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<uuid::Uuid>,
    /// Gets the group name.
    #[serde(rename = "GroupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::GroupStateType>,
    /// Gets the participants.
    #[serde(rename = "Participants", skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<String>>,
    /// Gets the date when this DTO has been created.
    #[serde(rename = "LastUpdatedAt", skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
}

impl GroupInfoDto {
    /// Class GroupInfoDto.
    pub fn new() -> GroupInfoDto {
        GroupInfoDto {
            group_id: None,
            group_name: None,
            state: None,
            participants: None,
            last_updated_at: None,
        }
    }
}
