/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetricsTimeSeries {
    /// Metrics Timestamps with values
    #[serde(rename = "values")]
    pub values: Vec<Vec<crate::models::MetricsTimeSeriesValue>>,
}

impl MetricsTimeSeries {
    #![allow(clippy::too_many_arguments)]
    pub fn new(values: Vec<Vec<crate::models::MetricsTimeSeriesValue>>) -> MetricsTimeSeries {
        MetricsTimeSeries { values }
    }
}
