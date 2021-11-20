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
pub struct LoadBalancerPrivateNet {
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<i32>,
}

impl LoadBalancerPrivateNet {
    pub fn new() -> LoadBalancerPrivateNet {
        LoadBalancerPrivateNet {
            ip: None,
            network: None,
        }
    }
}


