/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ListImagesResponse : Response to GET https://api.hetzner.cloud/v1/images

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListImagesResponse {
    #[serde(rename = "images")]
    pub images: Vec<crate::models::Image>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl ListImagesResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/images
    pub fn new(images: Vec<crate::models::Image>) -> ListImagesResponse {
        ListImagesResponse { images, meta: None }
    }
}
