/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerPrivateNet {
    #[serde(rename = "alias_ips", skip_serializing_if = "Option::is_none")]
    pub alias_ips: Option<Vec<String>>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "mac_address", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<i32>,
}

impl ServerPrivateNet {
    pub fn new() -> ServerPrivateNet {
        ServerPrivateNet {
            alias_ips: None,
            ip: None,
            mac_address: None,
            network: None,
        }
    }
}


