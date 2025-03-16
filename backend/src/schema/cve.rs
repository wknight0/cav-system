use serde::{Deserialize, Serialize};

// CVE struct holds name, sys_id, and values field for each CVE read in
use super::rank::CIAScore;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CVE {
    pub name: String,
    pub cve_id: String,
    pub host_address: Vec<String>,
    pub values: CIAScore,
}

impl CVE {
    // Constructor for CVE struct
    pub fn new(name: String, cve_id: String, host_address: String, values: CIAScore) -> CVE {
        CVE {
            name,
            cve_id,
            values,
            host_address: vec![host_address],
        }
    }

    // Get methods for individual fields
    pub fn retrieve_name(&self) -> &str {
        &self.name
    }

    pub fn retrieve_cve_id(&self) -> &str {
        &self.cve_id
    }

    pub fn retrieve_host_address(&self) -> &Vec<String> {
        &self.host_address
    }

    pub fn retrieve_values(&self) -> &CIAScore {
        &self.values
    }
}
