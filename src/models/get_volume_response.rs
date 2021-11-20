/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetVolumeResponse : Response to GET https://api.hetzner.cloud/v1/volumes/{id}



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetVolumeResponse {
    #[serde(rename = "volume")]
    pub volume: Box<crate::models::Volume>,
}

impl GetVolumeResponse {
    /// Response to GET https://api.hetzner.cloud/v1/volumes/{id}
    pub fn new(volume: crate::models::Volume) -> GetVolumeResponse {
        GetVolumeResponse {
            volume: Box::new(volume),
        }
    }
}


