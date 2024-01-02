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
pub enum Architecture {
    #[serde(rename = "X86")]
    X86,
    #[serde(rename = "X64")]
    X64,
    #[serde(rename = "Arm")]
    Arm,
    #[serde(rename = "Arm64")]
    Arm64,
    #[serde(rename = "Wasm")]
    Wasm,
    #[serde(rename = "S390x")]
    S390x,
}

impl ToString for Architecture {
    fn to_string(&self) -> String {
        match self {
            Self::X86 => String::from("X86"),
            Self::X64 => String::from("X64"),
            Self::Arm => String::from("Arm"),
            Self::Arm64 => String::from("Arm64"),
            Self::Wasm => String::from("Wasm"),
            Self::S390x => String::from("S390x"),
        }
    }
}

impl Default for Architecture {
    fn default() -> Architecture {
        Self::X86
    }
}
