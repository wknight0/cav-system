use cav_system::configuration::cve::importers::nessus_importer::NessusCveImporter;
use cav_system::configuration::cve::endpoints::nist_endpoint::NistEndpoint;
use cav_system::configuration::assets::assets_config::AssetsList;
use cav_system::schema::asset::{ Asset, AssetType, Properties };

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    let importer = NessusCveImporter;
    
    dotenv().ok();
    let endpoint = NistEndpoint::new(std::env::var("NIST_API_KEY").expect("NIST API KEY ENV VAR NOT FOUND"));
    let importer_results = importer.import(std::env::var("TEMP_IMPORT_PATH").expect("IMPORT CSV PATH ENV VAR NOT FOUND"), Box::new(endpoint)).await;
    
    
    let router = Asset::new(AssetType::Router, "192.168.1.113".to_string(), Properties::new(vec!["192.168.1.105".to_string()], "yes".to_string(), "true".to_string()));
    let desktop = Asset::new(AssetType::Router, "192.168.1.105".to_string(), Properties::new(vec!["192.168.1.113".to_string()], "yes".to_string(), "false".to_string()));
    let assets = AssetsList::create(vec![router, desktop]);
    
    println!("{:?}", importer_results);
    println!("{:?}", assets.retrieve_assets());
}
