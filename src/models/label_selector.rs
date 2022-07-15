/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.10.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// LabelSelector : Configuration for type LabelSelector, required if type is `label_selector`

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LabelSelector {
    /// Label selector
    #[serde(rename = "selector")]
    pub selector: String,
}

impl LabelSelector {
    #![allow(clippy::too_many_arguments)]
    /// Configuration for type LabelSelector, required if type is `label_selector`
    pub fn new(selector: String) -> LabelSelector {
        LabelSelector { selector }
    }
}
