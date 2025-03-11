use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;

use crate::configuration::cve::importers::cve_importer::CveImporter;
use crate::configuration::cve::endpoints::cve_endpoint::CveEndpoint;
use crate::schema::cve::CVE;
use crate::schema::rank::CIAScore;
pub struct NessusCveImporter;

#[async_trait::async_trait]
// Implements common CVEImporter traits from cve_importer interface into NessusCveImporter for import functionality
impl CveImporter for NessusCveImporter {
    async fn import(
        &self,
        file_path: String,
        endpoint: Box<dyn CveEndpoint + Send>,
    ) -> Result<Vec<CVE>, Box<dyn Error>> {
        self.import(file_path, endpoint).await
    }
}

// Nessus CVE Importer for collecting vector of CVE structs from data within exported nessus import csv files
impl NessusCveImporter {
    pub async fn import(
        &self,
        file_path: String,
        endpoint: Box<dyn CveEndpoint + Send>,
    ) -> Result<Vec<CVE>, Box<dyn Error>> {
        // Creates a CSV reader from the file path
        let mut rdr = ReaderBuilder::new().from_path(file_path.clone())?;

        // Reads the headers to find the indices of Name and CVE columns
        let headers = rdr.headers()?;
        let name_index = headers
            .iter()
            .position(|h| h == "Name")
            .ok_or("Name column not found")?;
        let id_index = headers
            .iter()
            .position(|h| h == "CVE")
            .ok_or("CVE column not found")?;
        let host_index = headers
            .iter()
            .position(|h| h == "Host")
            .ok_or("Host column not found")?;

        // Vector to read,store, and return CVE structs
        let mut cves: HashMap<String, CVE> = HashMap::new();

        for result in rdr.records() {
            let record = result?;

            // Checks to ensure that CVE ID is present and that it is not a duplicate value before pushing to cves vector
            if !(record.get(id_index).unwrap() == "") {
                let name = record.get(name_index).ok_or("Missing name")?.to_string();
                let cve_id = record.get(id_index).ok_or("Missing id")?.to_string();
                let host_address = record
                    .get(host_index)
                    .ok_or("Missing host address")?
                    .to_string();

                //if CVE already exists Apped host address to CVE
                if cves.contains_key(&cve_id) {
                    cves.get_mut(&cve_id)
                        .unwrap()
                        .host_address
                        .push(host_address);
                    continue;
                }

                //TODO: DECOUPLE THIS
                let values: CIAScore = endpoint.retrieve_cve_values(cve_id.clone()).await?;

                cves.insert(
                    cve_id.clone(),
                    CVE::new(name, cve_id.clone(), host_address, values),
                );
            }
        }
        Ok(cves.into_values().collect())
    }
}
