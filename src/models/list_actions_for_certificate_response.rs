/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListActionsForCertificateResponse : Response to GET https://api.hetzner.cloud/v1/certificates/{id}/actions



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListActionsForCertificateResponse {
    #[serde(rename = "actions")]
    pub actions: Vec<crate::models::Action>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl ListActionsForCertificateResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/certificates/{id}/actions
    pub fn new(actions: Vec<crate::models::Action>) -> ListActionsForCertificateResponse {
        ListActionsForCertificateResponse {
            actions,
            meta: None,
        }
    }
}


