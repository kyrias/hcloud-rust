/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetMetricsForServerResponse : Response to GET https://api.hetzner.cloud/v1/servers/{id}/metrics

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetMetricsForServerResponse {
    #[serde(rename = "metrics")]
    pub metrics: Box<crate::models::Metrics>,
}

impl GetMetricsForServerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/servers/{id}/metrics
    pub fn new(metrics: crate::models::Metrics) -> GetMetricsForServerResponse {
        GetMetricsForServerResponse {
            metrics: Box::new(metrics),
        }
    }
}
