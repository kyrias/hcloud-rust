/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.10.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerPublicNetFirewall {
    /// ID of the Resource
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Status of the Firewall on the Server
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl ServerPublicNetFirewall {
    #![allow(clippy::too_many_arguments)]
    pub fn new() -> ServerPublicNetFirewall {
        ServerPublicNetFirewall {
            id: None,
            status: None,
        }
    }
}

/// Status of the Firewall on the Server
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "applied")]
    Applied,
    #[serde(rename = "pending")]
    Pending,
}

impl Default for Status {
    fn default() -> Status {
        Self::Applied
    }
}
