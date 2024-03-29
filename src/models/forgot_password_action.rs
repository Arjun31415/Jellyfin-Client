/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ForgotPasswordAction {
    #[serde(rename = "ContactAdmin")]
    ContactAdmin,
    #[serde(rename = "PinCode")]
    PinCode,
    #[serde(rename = "InNetworkRequired")]
    InNetworkRequired,
}

impl ToString for ForgotPasswordAction {
    fn to_string(&self) -> String {
        match self {
            Self::ContactAdmin => String::from("ContactAdmin"),
            Self::PinCode => String::from("PinCode"),
            Self::InNetworkRequired => String::from("InNetworkRequired"),
        }
    }
}

impl Default for ForgotPasswordAction {
    fn default() -> ForgotPasswordAction {
        Self::ContactAdmin
    }
}
