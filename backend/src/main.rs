use cav_system::configuration::cve::importers::nessus_importer::NessusCveImporter;
use cav_system::configuration::cve::endpoints::nist_endpoint::NistEndpoint;
use cav_system::configuration::assets::assets_config::AssetsList;
use cav_system::schema::rank;
use cav_system::storage::db_handler;
use cav_system::analysis::ranking;
use cav_system::schema::asset::{ Asset, AssetType, Properties };
use cav_system::schema::storage::{ Credentials };
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // Initializes storage
    if let Err(e) = db_handler::initialize_storage() {
        eprintln!("Failed to initialize storage: {}", e);
        return;
    }

    /*
    // Gets temporary credentials values
    let nist_api_key = std::env::var("NIST_API_KEY").expect("NIST API KEY ENV VAR NOT FOUND");
    if let Err(e) = db_handler::update_credentials(Credentials::new(nist_api_key.clone())) {
        eprintln!("Failed to update credentials: {}", e);
        return;
    }

    // Imports CVE values
    dotenv().ok();
    let importer = NessusCveImporter;
    let endpoint = NistEndpoint::new(db_handler::retrieve_credentials().unwrap().nist_key);
    let importer_results = match importer.import(std::env::var("TEMP_IMPORT_PATH").expect("IMPORT CSV PATH ENV VAR NOT FOUND"), Box::new(endpoint)).await {
        Ok(results) => results,
        Err(e) => {
            eprintln!("Failed to import CVE values: {}", e);
            return;
        }
    };
    if let Err(e) = db_handler::update_cves(importer_results) {
        eprintln!("Failed to update CVEs: {}", e);
        return;
    }

    // Imports test Asset values
    let router = Asset::new(AssetType::Router, "192.168.1.113".to_string(), Properties::new(vec!["192.168.1.105".to_string()], "yes".to_string(), "true".to_string()));
    let desktop = Asset::new(AssetType::Router, "192.168.1.105".to_string(), Properties::new(vec!["192.168.1.113".to_string()], "yes".to_string(), "false".to_string()));
    let assets = AssetsList::create(vec![router, desktop]);
    if let Err(e) = db_handler::update_assets(assets.retrieve_assets()) {
        eprintln!("Failed to update Assets: {}", e);
        return;
    }
    */

    //println!("{:?}", db_handler::retrieve_credentials());
    //println!("{:?}", db_handler::retrieve_cves());
    //println!("{:?}", db_handler::retrieve_assets());

    let ranked_cves = ranking::rank_cves(db_handler::retrieve_cves().unwrap());
    println!("{:?}", ranked_cves);
}
