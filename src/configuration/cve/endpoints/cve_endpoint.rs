use std::error::Error;
use async_trait::async_trait;

use crate::schema::rank::CIAScore;

#[async_trait]
pub trait CveEndpoint {
    async fn retrieve_cve_values(&self, cve_id: String) -> Result<CIAScore, Box<dyn Error>>;
}
