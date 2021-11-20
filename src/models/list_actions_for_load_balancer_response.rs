/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListActionsForLoadBalancerResponse : Response to GET https://api.hetzner.cloud/v1/load_balancers/{id}/actions



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListActionsForLoadBalancerResponse {
    #[serde(rename = "actions")]
    pub actions: Vec<crate::models::Action>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl ListActionsForLoadBalancerResponse {
    /// Response to GET https://api.hetzner.cloud/v1/load_balancers/{id}/actions
    pub fn new(actions: Vec<crate::models::Action>) -> ListActionsForLoadBalancerResponse {
        ListActionsForLoadBalancerResponse {
            actions,
            meta: None,
        }
    }
}


