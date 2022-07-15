/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.10.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// IpType : The type of the IP

/// The type of the IP
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IpType {
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    Ipv6,
}

impl ToString for IpType {
    fn to_string(&self) -> String {
        match self {
            Self::Ipv4 => String::from("ipv4"),
            Self::Ipv6 => String::from("ipv6"),
        }
    }
}

impl Default for IpType {
    fn default() -> IpType {
        Self::Ipv4
    }
}
