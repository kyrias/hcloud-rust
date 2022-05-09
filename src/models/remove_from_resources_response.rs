/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RemoveFromResourcesResponse : Response to POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/remove_from_resources



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveFromResourcesResponse {
    #[serde(rename = "actions")]
    pub actions: Vec<crate::models::Action>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl RemoveFromResourcesResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/remove_from_resources
    pub fn new(actions: Vec<crate::models::Action>) -> RemoveFromResourcesResponse {
        RemoveFromResourcesResponse {
            actions,
            meta: None,
        }
    }
}


