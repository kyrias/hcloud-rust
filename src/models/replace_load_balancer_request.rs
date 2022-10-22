/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceLoadBalancerRequest : Request for PUT https://api.hetzner.cloud/v1/load_balancers/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplaceLoadBalancerRequest {
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// New Load Balancer name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ReplaceLoadBalancerRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for PUT https://api.hetzner.cloud/v1/load_balancers/{id}
    pub fn new() -> ReplaceLoadBalancerRequest {
        ReplaceLoadBalancerRequest {
            labels: None,
            name: None,
        }
    }
}
