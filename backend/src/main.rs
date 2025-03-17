use cav_system::controller;
use cav_system::schema::asset::Asset;
use cav_system::schema::cve::CVE;
use cav_system::schema::rank::RankedCVE;
use cav_system::schema::storage::Credentials;

// Tauri command to initialize storage using controller
#[tauri::command]
async fn initialize_storage() -> bool {
    match controller::initialize_storage().await {
        Ok(_results) => true,
        Err(_e) => false
    }
}

// Tauri command to remove credentials using controller
#[tauri::command]
async fn remove_credentials() -> bool {
    match controller::remove_credentials().await {
        Ok(_results) => true,
        Err(_e) => false
    }
}

// Tauri command to remove cves using controller
#[tauri::command]
async fn remove_cves() -> bool {
    match controller::remove_cves().await {
        Ok(_results) => true,
        Err(_e) => false
    }
}

// Tauri command to remove assets using controller
#[tauri::command]
async fn remove_assets() -> bool {
    match controller::remove_assets().await {
        Ok(_results) => true,
        Err(_e) => false
    }
}

// Tauri command to remove ranked cves using controller
#[tauri::command]
async fn remove_ranked_cves() -> bool {
    match controller::remove_ranked_cves().await {
        Ok(_results) => true,
        Err(_e) => false
    }
}

// Tauri command to retrieve credentials using controller
#[tauri::command]
async fn retrieve_credentials() -> Result<Credentials, String> {
    match controller::retrieve_credentials().await {
        Ok(credentials) => Ok(credentials),
        Err(e) => Err(format!("Failed to retrieve credentials: {}", e)),
    }
}

// Tauri command to retrieve cves using controller
#[tauri::command]
async fn retrieve_cves() -> Result<Vec<CVE>, String> {
    match controller::retrieve_cves().await {
        Ok(cves) => Ok(cves),
        Err(e) => Err(format!("Failed to retrieve cves: {}", e)),
    }
}

// Tauri command to retrieve assets using controller
#[tauri::command]
async fn retrieve_assets() -> Result<Vec<Asset>, String> {
    match controller::retrieve_assets().await {
        Ok(assets) => Ok(assets),
        Err(e) => Err(format!("Failed to retrieve assets: {}", e)),
    }
}

// Tauri command to retrieve ranked cves using controller
#[tauri::command]
async fn retrieve_ranked_cves() -> Result<Vec<RankedCVE>, String> {
    match controller::retrieve_ranked_cves().await {
        Ok(ranked_cves) => Ok(ranked_cves),
        Err(e) => Err(format!("Failed to retrieve ranked cves: {}", e)),
    }
}

// Tauri command to update credentials using controller
#[tauri::command]
async fn update_credentials(credentials: Credentials) -> bool {
    match controller::update_credentials(credentials).await {
        Ok(_results) => true,
        Err(_e) => false
    }
}

// Tauri command to update cves using controller
#[tauri::command]
async fn update_cves(file_path: String) -> bool {
    match controller::update_cves(file_path).await {
        Ok(_results) => true,
        Err(_e) => false
    }
}

// Tauri command to update assets using controller
#[tauri::command]
async fn update_assets(assets: Vec<Asset>) -> bool {
    match controller::update_assets(assets).await {
        Ok(_results) => true,
        Err(_e) => false
    }
}

// Tauri command to update ranked cves using controller
#[tauri::command]
async fn update_ranked_cves() -> bool {
    match controller::update_ranked_cves().await {
        Ok(_results) => true,
        Err(_e) => false
    }
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            initialize_storage,
            remove_credentials,
            remove_cves,
            remove_assets,
            remove_ranked_cves,
            retrieve_credentials,
            retrieve_cves,
            retrieve_assets,
            retrieve_ranked_cves,
            update_credentials,
            update_cves,
            update_assets,
            update_ranked_cves
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
