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
    Router,
    Switch,
    Server,
    Desktop,
    Laptop,
    MobileDevice,
    IoTDevice,
}

// Position struct which holds where the asset is located on topology
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

// Properties struct which holds contextual variables relevant to the asset
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Properties {
    pub relations: Vec<String>,
    pub firewall_status: String,
    pub internet_facing: String,
}

impl Properties {
    // Constructor for the Properties struct
    pub fn new(
        relations: Vec<String>,
        firewall_status: String,
        internet_facing: String,
    ) -> Properties {
        Properties {
            relations,
            firewall_status,
            internet_facing,
        }
    }

    // Get methods for individual fields
    pub fn retrieve_relations(&self) -> Vec<String> {
        self.relations.clone()
    }

    pub fn retrieve_firewall_status(&self) -> &str {
        &self.firewall_status
    }

    pub fn retrieve_internet_facing(&self) -> &str {
        &self.internet_facing
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
