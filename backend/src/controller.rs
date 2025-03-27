use std::error::Error;

use crate::analysis::ranking;
use crate::configuration::cve::importers::nessus_importer::NessusCveImporter;
use crate::configuration::cve::endpoints::nist_endpoint::NistEndpoint;
use crate::schema::storage::Credentials;
use crate::schema::cve::CVE;
use crate::schema::asset::Asset;
use crate::schema::rank::RankedCVE;
use crate::storage::db_handler;

#[tokio::main]

pub async fn test_function() -> Result<(), Box<dyn Error>> {
    Ok(())
}

// Initializes local storage for keeping state of credentials, cves, assets, and ranked cves
pub async fn initialize_storage() -> Result<(), Box<dyn Error>> {
    match db_handler::initialize_storage() {
        Ok(_) => {
            println!("Storage successfully initialized...");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to initialize storage: {}", e);
            Err(e)
        }
    }
}

// Remove functions for credentials, cves, assets, and ranked cves
pub async fn remove_credentials() -> Result<(), Box<dyn Error>> {
    match db_handler::remove_credentials() {
        Ok(_) => {
            println!("Credentials removed successfully...");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to remove credentials: {}", e);
            Err(e)
        }
    }
}

pub async fn remove_cves() -> Result<(), Box<dyn Error>> {
    match db_handler::remove_cves() {
        Ok(_) => {
            println!("Cves removed successfully...");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to remove cves: {}", e);
            Err(e)
        }
    }
}

pub async fn remove_assets() -> Result<(), Box<dyn Error>> {
    match db_handler::remove_assets() {
        Ok(_) => {
            println!("Assets removed successfully...");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to remove assets: {}", e);
            Err(e)
        }
    }
}

pub async fn remove_ranked_cves() -> Result<(), Box<dyn Error>> {
    match db_handler::remove_ranked_cves() {
        Ok(_) => {
            println!("Ranked cves removed successfully...");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to remove ranked cves: {}", e);
            Err(e)
        }
    }
}

// Retrieve functions for credentials, cves, assets, and ranked cves
pub async fn retrieve_credentials() -> Result<Credentials, Box<dyn Error>> {
    match db_handler::retrieve_credentials() {
        Ok(credentials) => {
            println!("Credentials retrieved successfully...");
            Ok(credentials)
        }
        Err(e) => {
            eprintln!("Failed to retrieve credentials: {}", e);
            Err(e)
        }
    }
}

pub async fn retrieve_cves() -> Result<Vec<CVE>, Box<dyn Error>> {
    match db_handler::retrieve_cves() {
        Ok(cves) => {
            println!("Cves retrieved successfully...");
            Ok(cves)
        }
        Err(e) => {
            eprintln!("Failed to retrieve cves: {}", e);
            Err(e)
        }
    }
}

pub async fn retrieve_assets() -> Result<Vec<Asset>, Box<dyn Error>> {
    match db_handler::retrieve_assets() {
        Ok(assets) => {
            println!("Assets retrieved successfully...");
            Ok(assets)
        }
        Err(e) => {
            eprintln!("Failed to retrieve assets: {}", e);
            Err(e)
        }
    }
}

pub async fn retrieve_ranked_cves() -> Result<Vec<RankedCVE>, Box<dyn Error>> {
    match db_handler::retrieve_ranked_cves() {
        Ok(ranked_cves) => {
            println!("Ranked cves retrieved successfully...");
            Ok(ranked_cves)
        }
        Err(e) => {
            eprintln!("Failed to retrieve ranked cves: {}", e);
            Err(e)
        }
    }
}

// Update credentials function
pub async fn update_credentials(credentials: Credentials) -> Result<(), Box<dyn Error>> {
    match db_handler::update_credentials(credentials) {
        Ok(_) => {
            println!("Credentials updated successfully...");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to update credentials: {}", e);
            Err(e)
        }
    }
}

// Loads cves and updates the local storage
pub async fn update_cves(file_path: String) -> Result<(), Box<dyn Error>> {
    let importer = NessusCveImporter;
    
    // Retrieves credentials, handling any errors if they occur
    let nist_key = match db_handler::retrieve_credentials() {
        Ok(credentials) => credentials.nist_key,
        Err(e) => {
            eprintln!("Failed to retrieve credentials: {}", e);
            return Err(e.into());
        }
    };

    let endpoint = NistEndpoint::new(nist_key);

    // Import CVE from REST API calls and CSV file
    let cve_import_results = importer
        .import(file_path.clone(), Box::new(endpoint))
        .await
        .map_err(|e| {
            eprintln!("Failed to import CVEs from file {}: {}", file_path, e);
            e
        })?;
    
    if cve_import_results.is_empty() {
        let error_message = format!("No CVEs were imported from file {}", file_path);
        eprintln!("{}", error_message);
        return Err(error_message.into());
    }

    // Update CVE vector in local database
    db_handler::update_cves(cve_import_results.clone()).map_err(|e| {
        eprintln!("Failed to save CVE results into local database: {}", e);
        e
    })?;

    println!("CVE results saved successfully...");
    Ok(())
}

// Updates assets for the local storage
pub async fn update_assets(assets: Vec<Asset>) -> Result<(), Box<dyn Error>> {
    match db_handler::update_assets(assets) {
        Ok(_) => {
            println!("Asset results saved successfully...");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to save asset results into local database: {}", e);
            Err(e)
        }
    }
}

// Ranks cves and updates the local storage with ranked cves
pub async fn update_ranked_cves() -> Result<(), Box<dyn Error>> {
    let cves: Vec<CVE>;
    let assets: Vec<Asset>;
    let ranked_cves: Vec<RankedCVE>;

    // Retrieve cves
    cves = match db_handler::retrieve_cves() {
        Ok(cves) => cves,
        Err(e) => {
            eprintln!("Failed to retrieve cves: {}", e);
            return Err(e);
        }
    };

    // Retrieve assets
    assets = match db_handler::retrieve_assets() {
        Ok(assets) => assets,
        Err(e) => {
            eprintln!("Failed to retrieve assets: {}", e);
            return Err(e);
        }
    };

    // Rank cves based on cves and assets
    ranked_cves = ranking::rank_cves(cves, assets);

    // Update ranked cves in local database
    match db_handler::update_ranked_cves(ranked_cves) {
        Ok(_) => {
            println!("Ranked cve results saved successfully...");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to save ranked cve results into local database: {}", e);
            Err(e)
        }
    }
}
