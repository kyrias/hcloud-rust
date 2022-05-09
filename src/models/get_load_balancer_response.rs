/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetLoadBalancerResponse : Response to GET https://api.hetzner.cloud/v1/load_balancers/{id}



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetLoadBalancerResponse {
    #[serde(rename = "load_balancer")]
    pub load_balancer: Box<crate::models::LoadBalancer>,
}

impl GetLoadBalancerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/load_balancers/{id}
    pub fn new(load_balancer: crate::models::LoadBalancer) -> GetLoadBalancerResponse {
        GetLoadBalancerResponse {
            load_balancer: Box::new(load_balancer),
        }
    }
}


