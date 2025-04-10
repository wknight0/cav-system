use serde::{Deserialize, Serialize};

// Asset struct holds asset type, ip address, and properties for context in the network
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    pub _id: String,
    pub label: String,
    pub asset_type: AssetType,
    pub position: Position,
    pub ip_address: String,
    pub properties: Properties,
}

impl Asset {
    // Constructor for the Asset struct
    pub fn new(_id: String, label: String, asset_type: AssetType, position: Position, ip_address: String, properties: Properties) -> Self {
        Asset {
            _id,
            label,
            asset_type,
            position,
            ip_address,
            properties,
        }
    }

    // Get methods for individual fields
    pub fn retrieve_asset_type(&self) -> AssetType {
        self.asset_type
    }

    pub fn retrieve_ip_address(&self) -> &str {
        &self.ip_address
    }
}

// Asset type enum which holds the different types of assets in the network
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum AssetType {
    Switch,
    Router,
    Firewall,
    LoadBalancer,
    Server,
    Storage,
    Desktop,
    Laptop,
    MobileDevice,
    Printer,
    IoTDevice,
}

// Position struct which holds where the asset is located on topology
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

// Properties struct which holds contextual variables relevant to the asset
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Properties {
    pub operating_system: String,
    pub internet_facing: String,
    pub firewall_status: String,
    pub firmware_status: String,
    pub tamper_detection_status: String,
    pub vpn_support: String,
    pub vlan_support: String,
    pub poe_support: String,
    pub encryption_enabled: String,
    pub virtualization_enabled: String,
    pub network_enabled: String,
    pub health_check_enabled: String,
}

impl Properties {
    // Constructor for the Properties struct
    pub fn new(
        operating_system: String,
        internet_facing: String,
        firewall_status: String,
        firmware_status: String,
        tamper_detection_status: String,
        vpn_support: String,
        vlan_support: String,
        poe_support: String,
        encryption_enabled: String,
        virtualization_enabled: String,
        network_enabled: String,
        health_check_enabled: String,
    ) -> Properties {
        Properties {
            operating_system,
            internet_facing,
            firewall_status,
            firmware_status,
            tamper_detection_status,
            vpn_support,
            vlan_support,
            poe_support,
            encryption_enabled,
            virtualization_enabled,
            network_enabled,
            health_check_enabled,
        }
    }

    // Get methods for individual fields
    pub fn retrieve_operating_system(&self) -> &str {
        &self.operating_system
    }
    
    pub fn retrieve_internet_facing(&self) -> &str {
        &self.internet_facing
    }

    pub fn retrieve_firewall_status(&self) -> &str {
        &self.firewall_status
    }

    pub fn retrieve_firmware_status(&self) -> &str {
        &self.firmware_status
    }

    pub fn retrieve_tamper_detection_status(&self) -> &str {
        &self.tamper_detection_status
    }

    pub fn retrieve_vpn_support(&self) -> &str {
        &self.vpn_support
    }

    pub fn retrieve_vlan_support(&self) -> &str {
        &self.vlan_support
    }

    pub fn retrieve_poe_support(&self) -> &str {
        &self.poe_support
    }

    pub fn retrieve_encryption_enabled(&self) -> &str {
        &self.encryption_enabled
    }

    pub fn retrieve_virtualization_enabled(&self) -> &str {
        &self.virtualization_enabled
    }

    pub fn retrieve_network_enabled(&self) -> &str {
        &self.network_enabled
    }

    pub fn retrieve_health_check_enabled(&self) -> &str {
        &self.health_check_enabled
    }
}

// Connection struct holds links between assets in the network
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connection {
    pub _id: String,
    pub connection_type: String,
    pub source: String,
    pub destination: String,
}

impl Connection {
    // Constructor for the Connection struct
    pub fn new(_id: String, connection_type: String, source: String, destination: String) -> Self {
        Connection {
            _id,
            connection_type,
            source,
            destination,
        }
    }
}
