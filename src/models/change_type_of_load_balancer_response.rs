/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChangeTypeOfLoadBalancerResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_type



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeTypeOfLoadBalancerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ChangeTypeOfLoadBalancerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_type
    pub fn new(action: crate::models::Action) -> ChangeTypeOfLoadBalancerResponse {
        ChangeTypeOfLoadBalancerResponse {
            action: Box::new(action),
        }
    }
}


