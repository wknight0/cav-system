use crate::schema::asset::{Asset, Connection};
use crate::schema::cve::CVE;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Storage {
    pub key: String,
    pub credentials: Credentials,
    pub cves: Vec<CVE>,
    pub assets: Vec<Asset>,
    pub connections: Vec<Connection>,
}

impl Storage {
    // Constructor for the Storage struct
    pub fn new(key: String, credentials: Credentials) -> Storage {
        let cves = Vec::new();
        let assets = Vec::new();
        let connections = Vec::new();
        Storage {
            key,
            credentials,
            cves,
            assets,
            connections,
        }
    }

    // Get methods for individual fields
    pub fn retrieve_key(&self) -> &String {
        &self.key
    }

    pub fn retrieve_credentials(&self) -> &Credentials {
        &self.credentials
    }

    pub fn retrieve_cves(&self) -> &Vec<CVE> {
        &self.cves
    }

    pub fn retrieve_assets(&self) -> &Vec<Asset> {
        &self.assets
    }

    pub fn retrieve_connections(&self) -> &Vec<Connection> {
        &self.connections
    }

    // Set methods for individual fields
    pub fn update_key(&mut self, key: String) {
        self.key = key;
    }

    pub fn update_credentials(&mut self, credentials: Credentials) {
        self.credentials = credentials;
    }
    
    pub fn update_cves(&mut self, cves: Vec<CVE>) {
        self.cves = cves;
    }

    pub fn update_assets(&mut self, assets: Vec<Asset>) {
        self.assets = assets;
    }

    pub fn update_connections(&mut self, connections: Vec<Connection>) {
        self.connections = connections;
    }
}

// Credentials struct which holds the different variables read from the api call for endpoint
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub nist_key: String,
}

impl Credentials {
    // Constructor for the Credentials struct
    pub fn new(
        nist_key: String,
    ) -> Credentials {
        Credentials {
            nist_key,
        }
    }
}
