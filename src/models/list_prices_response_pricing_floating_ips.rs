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
pub struct ListPricesResponsePricingFloatingIps {
    /// Floating IP type costs per Location
    #[serde(rename = "prices")]
    pub prices: Vec<crate::models::PricePerTimeMonthly>,
    #[serde(rename = "type")]
    pub r#type: crate::models::IpType,
}

impl ListPricesResponsePricingFloatingIps {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        prices: Vec<crate::models::PricePerTimeMonthly>,
        r#type: crate::models::IpType,
    ) -> ListPricesResponsePricingFloatingIps {
        ListPricesResponsePricingFloatingIps { prices, r#type }
    }
}
