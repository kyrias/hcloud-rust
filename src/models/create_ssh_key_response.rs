/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateSshKeyResponse : Response to POST https://api.hetzner.cloud/v1/ssh_keys

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateSshKeyResponse {
    #[serde(rename = "ssh_key")]
    pub ssh_key: Box<crate::models::SshKey>,
}

impl CreateSshKeyResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/ssh_keys
    pub fn new(ssh_key: crate::models::SshKey) -> CreateSshKeyResponse {
        CreateSshKeyResponse {
            ssh_key: Box::new(ssh_key),
        }
    }
}
