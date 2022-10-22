/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// SshKey : SSH keys are public keys you provide to the cloud system. They can be injected into Servers at creation time. We highly recommend that you use keys instead of passwords to manage your Servers.

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SshKey {
    /// Point in time when the Resource was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
    /// Fingerprint of public key
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,
    /// ID of the Resource
    #[serde(rename = "id")]
    pub id: i32,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    /// Name of the Resource. Must be unique per Project.
    #[serde(rename = "name")]
    pub name: String,
    /// Public key
    #[serde(rename = "public_key")]
    pub public_key: String,
}

impl SshKey {
    #![allow(clippy::too_many_arguments)]
    /// SSH keys are public keys you provide to the cloud system. They can be injected into Servers at creation time. We highly recommend that you use keys instead of passwords to manage your Servers.
    pub fn new(
        created: String,
        fingerprint: String,
        id: i32,
        labels: ::std::collections::HashMap<String, String>,
        name: String,
        public_key: String,
    ) -> SshKey {
        SshKey {
            created,
            fingerprint,
            id,
            labels,
            name,
            public_key,
        }
    }
}
