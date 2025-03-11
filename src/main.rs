use hava_system::configuration::cve::importers::nessus_importer::NessusCveImporter;
use hava_system::configuration::cve::endpoints::nist_endpoint::NistEndpoint;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    let importer = NessusCveImporter;
    
    dotenv().ok();
    let endpoint = NistEndpoint::new(std::env::var("NIST_API_KEY").expect("NIST API KEY ENV VAR NOT FOUND"));
    let importer_results = importer.import(std::env::var("TEMP_IMPORT_PATH").expect("IMPORT CSV PATH ENV VAR NOT FOUND"), Box::new(endpoint)).await;
    
    println!("{:?}", importer_results);
}
