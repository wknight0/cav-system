use std::error::Error;

use crate::{configuration::cve::endpoints::cve_endpoint::CveEndpoint, schema::cve::CVE};

#[async_trait::async_trait]
// Interface to be implemented for specific queries to retrieve list of CVEs
pub trait CveImporter {
    // Checks to ensure that CVEs have been parsed successfully after retrieving CVEs from csv, and returns CVEs vector
    async fn import(
        &self,
        file_path: String,
        endpoint: Box<dyn CveEndpoint + Send>,
    ) -> Result<Vec<CVE>, Box<dyn Error>>;
}
