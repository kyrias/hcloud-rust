/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.10.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoadBalancerType {
    /// Point in time when the Load Balancer type is deprecated (in ISO-8601 format)
    #[serde(rename = "deprecated")]
    pub deprecated: Option<String>,
    /// Description of the Load Balancer type
    #[serde(rename = "description")]
    pub description: String,
    /// ID of the Load Balancer type
    #[serde(rename = "id")]
    pub id: i32,
    /// Number of SSL Certificates that can be assigned to a single Load Balancer
    #[serde(rename = "max_assigned_certificates")]
    pub max_assigned_certificates: i32,
    /// Number of maximum simultaneous open connections
    #[serde(rename = "max_connections")]
    pub max_connections: i32,
    /// Number of services a Load Balancer of this type can have
    #[serde(rename = "max_services")]
    pub max_services: i32,
    /// Number of targets a single Load Balancer can have
    #[serde(rename = "max_targets")]
    pub max_targets: i32,
    /// Unique identifier of the Load Balancer type
    #[serde(rename = "name")]
    pub name: String,
    /// Prices in different network zones
    #[serde(rename = "prices")]
    pub prices: Vec<crate::models::PricePerTime>,
}

impl LoadBalancerType {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        deprecated: Option<String>,
        description: String,
        id: i32,
        max_assigned_certificates: i32,
        max_connections: i32,
        max_services: i32,
        max_targets: i32,
        name: String,
        prices: Vec<crate::models::PricePerTime>,
    ) -> LoadBalancerType {
        LoadBalancerType {
            deprecated,
            description,
            id,
            max_assigned_certificates,
            max_connections,
            max_services,
            max_targets,
            name,
            prices,
        }
    }
}
