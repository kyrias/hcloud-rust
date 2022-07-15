/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.10.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ListActionsForFirewallResponse : Response to GET https://api.hetzner.cloud/v1/firewalls/{id}/actions

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListActionsForFirewallResponse {
    #[serde(rename = "actions")]
    pub actions: Vec<crate::models::Action>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl ListActionsForFirewallResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/firewalls/{id}/actions
    pub fn new(actions: Vec<crate::models::Action>) -> ListActionsForFirewallResponse {
        ListActionsForFirewallResponse {
            actions,
            meta: None,
        }
    }
}
