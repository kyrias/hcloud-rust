/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Server : Servers are virtual machines that can be provisioned.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Server {
    /// Time window (UTC) in which the backup will run, or null if the backups are not enabled
    #[serde(rename = "backup_window")]
    pub backup_window: Option<String>,
    /// Point in time when the Resource was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "datacenter")]
    pub datacenter: Box<crate::models::Datacenter>,
    /// ID of the Resource
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "image")]
    pub image: Option<Box<crate::models::Image>>,
    /// Free Traffic for the current billing period in bytes
    #[serde(rename = "included_traffic")]
    pub included_traffic: Option<i64>,
    /// Inbound Traffic for the current billing period in bytes
    #[serde(rename = "ingoing_traffic")]
    pub ingoing_traffic: Option<i64>,
    #[serde(rename = "iso")]
    pub iso: Option<Box<crate::models::Iso>>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    #[serde(rename = "load_balancers", skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<i32>>,
    /// True if Server has been locked and is not available to user
    #[serde(rename = "locked")]
    pub locked: bool,
    /// Name of the Resource. Must be unique per Project.
    #[serde(rename = "name")]
    pub name: String,
    /// Outbound Traffic for the current billing period in bytes
    #[serde(rename = "outgoing_traffic")]
    pub outgoing_traffic: Option<i64>,
    #[serde(rename = "placement_group", skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<Box<crate::models::PlacementGroup>>,
    /// Size of the primary Disk
    #[serde(rename = "primary_disk_size")]
    pub primary_disk_size: i32,
    /// Private networks information
    #[serde(rename = "private_net")]
    pub private_net: Vec<crate::models::ServerPrivateNet>,
    #[serde(rename = "protection")]
    pub protection: Box<crate::models::ServerProtection>,
    #[serde(rename = "public_net")]
    pub public_net: Box<crate::models::ServerPublicNet>,
    /// True if rescue mode is enabled. Server will then boot into rescue system on next reboot
    #[serde(rename = "rescue_enabled")]
    pub rescue_enabled: bool,
    #[serde(rename = "server_type")]
    pub server_type: Box<crate::models::ServerType>,
    /// Status of the Server
    #[serde(rename = "status")]
    pub status: Status,
    /// IDs of Volumes assigned to this Server
    #[serde(rename = "volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<i32>>,
}

impl Server {
    #![allow(clippy::too_many_arguments)]
    /// Servers are virtual machines that can be provisioned.
    pub fn new(backup_window: Option<String>, created: String, datacenter: crate::models::Datacenter, id: i32, image: Option<crate::models::Image>, included_traffic: Option<i64>, ingoing_traffic: Option<i64>, iso: Option<crate::models::Iso>, labels: ::std::collections::HashMap<String, String>, locked: bool, name: String, outgoing_traffic: Option<i64>, primary_disk_size: i32, private_net: Vec<crate::models::ServerPrivateNet>, protection: crate::models::ServerProtection, public_net: crate::models::ServerPublicNet, rescue_enabled: bool, server_type: crate::models::ServerType, status: Status) -> Server {
        Server {
            backup_window,
            created,
            datacenter: Box::new(datacenter),
            id,
            image: image.map(Box::new),
            included_traffic,
            ingoing_traffic,
            iso: iso.map(Box::new),
            labels,
            load_balancers: None,
            locked,
            name,
            outgoing_traffic,
            placement_group: None,
            primary_disk_size,
            private_net,
            protection: Box::new(protection),
            public_net: Box::new(public_net),
            rescue_enabled,
            server_type: Box::new(server_type),
            status,
            volumes: None,
        }
    }
}

/// Status of the Server
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "migrating")]
    Migrating,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "rebuilding")]
    Rebuilding,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for Status {
    fn default() -> Status {
        Self::Deleting
    }
}

