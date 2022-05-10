/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DetachIsoFromServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/detach_iso



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DetachIsoFromServerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl DetachIsoFromServerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/detach_iso
    pub fn new(action: crate::models::Action) -> DetachIsoFromServerResponse {
        DetachIsoFromServerResponse {
            action: Box::new(action),
        }
    }
}


