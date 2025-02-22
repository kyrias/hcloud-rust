/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetIsoResponse : Response to GET https://api.hetzner.cloud/v1/isos/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetIsoResponse {
    #[serde(rename = "iso")]
    pub iso: Box<crate::models::Iso>,
}

impl GetIsoResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/isos/{id}
    pub fn new(iso: crate::models::Iso) -> GetIsoResponse {
        GetIsoResponse { iso: Box::new(iso) }
    }
}
