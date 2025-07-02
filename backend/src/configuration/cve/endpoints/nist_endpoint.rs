use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use async_trait::async_trait;
use serde_json::Value;

use crate::schema::rank::CIAScore;
use crate::configuration::cve::endpoints::cve_endpoint::CveEndpoint;

pub struct NistEndpoint {
    key: String,
}

impl NistEndpoint {
    pub fn new(key: String) -> NistEndpoint {
        NistEndpoint { key }
    }
}

#[async_trait]
impl CveEndpoint for NistEndpoint {
    // Retrieves vector string from NIST call, utilizing api key provided and returns Values struct for CVE based on stored values
    async fn retrieve_cve_values(&self, cve_id: String) -> Result<CIAScore, Box<dyn Error>> {
        let client = reqwest::Client::builder().build()?;
        let request = client.request(
            reqwest::Method::GET,
            format!(
                "https://services.nvd.nist.gov/rest/json/cves/2.0?cveId={}",
                cve_id
            ),
        );
        let response = request.header("apiKey", self.key.clone()).send().await?;

        // 0.6s sleep to avoid rate limiting from Nist Endpoint
        sleep(Duration::from_secs_f32(0.6));

        if response.status().is_success() {
            println!("Obtaining {} values...", cve_id);
            // Iterates through JSON response to obtain Vector String and separates into individual values
            let v: Value = response.json().await?;
            let values: CIAScore;
            let confidentiality_value: f32;
            let integrity_value: f32;
            let availability_value: f32;
            let vector_string: String;
            let vector: Vec<&str>;
            let mut string_vectors: Vec<&str> = Vec::new();

            if v["vulnerabilities"][0]["cve"]["metrics"]["cvssMetricV31"].is_null() {
                vector_string = v["vulnerabilities"][0]["cve"]["metrics"]["cvssMetricV2"][0]
                    ["cvssData"]["vectorString"]
                    .to_string();
                vector = vector_string.split('/').collect();

                string_vectors.push(vector[3]);
                string_vectors.push(vector[4]);
                string_vectors.push(vector[5]);
            } else {
                vector_string = v["vulnerabilities"][0]["cve"]["metrics"]["cvssMetricV31"][0]
                    ["cvssData"]["vectorString"]
                    .to_string();
                vector = vector_string.split('/').collect();

                string_vectors.push(vector[6]);
                string_vectors.push(vector[7]);
                string_vectors.push(vector[8]);
            }

            if string_vectors.get(0).unwrap() == &"C:C".to_string() {
                confidentiality_value = 0.660;
            } else if string_vectors.get(1).unwrap() == &"C:P".to_string() {
                confidentiality_value = 0.275;
            } else {
                confidentiality_value = 0.0;
            }

            if string_vectors.get(1).unwrap() == &"I:C".to_string() {
                integrity_value = 0.660;
            } else if string_vectors.get(1).unwrap() == &"I:P".to_string() {
                integrity_value = 0.275;
            } else {
                integrity_value = 0.0;
            }

            if string_vectors.get(2).unwrap() == &"A:C\"".to_string() {
                availability_value = 0.660;
            } else if *string_vectors.get(2).unwrap() == &"A:P\"".to_string() {
                availability_value = 0.275;
            } else {
                availability_value = 0.0;
            }

            values = CIAScore::new(confidentiality_value, integrity_value, availability_value);

            Ok(values)
        } else {
            // Handle non-success status codes
            let status = response.status().as_u16();
            let body = response.text().await?;
            Err(format!(
                "Failed to fetch CVEs for {}. Status: {}, Body: {}",
                cve_id, status, body
            )
            .into())
        }
    }
}
