use std::error::Error;
use std::path::PathBuf;
use std::fs;

use bson::to_bson;
use polodb_core::bson::doc;
use polodb_core::Database;

use crate::schema::asset::{Asset, Connection};
use crate::schema::cve::CVE;
use crate::schema::storage::{Storage, Credentials};

// Initializes new local storage instance and inserts into local database if it doesn't exist
pub fn initialize_storage() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = String::from("./cache_storage/local");
    let db_dir = PathBuf::from("./cache_storage");

    if !db_dir.exists() {
        fs::create_dir_all(&db_dir).expect("Failed to create database directory");
    }
    
    let db = Database::open_file(db_path)?;
    let collection = db.collection::<Storage>("local");

    let filter = doc! { "key": "local" };

    if collection.find_one(filter.clone())?.is_none() {
        let storage = Storage {
            key: "local".to_string(),
            credentials: Credentials::new("".to_string()),
            cves: Vec::new(),
            assets: Vec::new(),
            connections: Vec::new(),
        };
        collection.insert_one(storage)?;
    }
    Ok(())
}

// Removes credentials from the local storage
pub fn remove_credentials() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = String::from("./cache_storage/local");
    let db = Database::open_file(db_path)?;
    let collection = db.collection::<Storage>("local");

    let filter = doc! { "key": "local" };

    if let Some(_) = collection.find_one(filter.clone())? {
        collection.update_one(filter, doc! { "$unset": { "credentials": "" } })?;
        Ok(())
    } else {
        Err(Box::from("Storage not found..."))
    }
}

// Removes CVEs from the local storage
pub fn remove_cves() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = String::from("./cache_storage/local");
    let db = Database::open_file(db_path)?;
    let collection = db.collection::<Storage>("local");

    let filter = doc! { "key": "local" };

    if let Some(_) = collection.find_one(filter.clone())? {
        collection.update_one(filter, doc! { "$unset": { "cves": "" } })?;
        Ok(())
    } else {
        Err(Box::from("Storage not found..."))
    }
}

// Removes assets from the local storage
pub fn remove_assets() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = String::from("./cache_storage/local");
    let db = Database::open_file(db_path)?;
    let collection = db.collection::<Storage>("local");

    let filter = doc! { "key": "local" };

    if let Some(_) = collection.find_one(filter.clone())? {
        collection.update_one(filter, doc! { "$unset": { "assets": "" } })?;
        Ok(())
    } else {
        Err(Box::from("Storage not found..."))
    }
}

// Removes connections from the local storage
pub fn remove_connections() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = String::from("./cache_storage/local");
    let db = Database::open_file(db_path)?;
    let collection = db.collection::<Storage>("local");

    let filter = doc! { "key": "local" };

    if let Some(_) = collection.find_one(filter.clone())? {
        collection.update_one(filter, doc! { "$unset": { "connections": "" } })?;
        Ok(())
    } else {
        Err(Box::from("Storage not found..."))
    }
}

// Retrieve methods for credentials, cves, assets
pub fn retrieve_credentials() -> Result<Credentials, Box<dyn std::error::Error>> {
    let db_path = String::from("./cache_storage/local");
    let db = Database::open_file(db_path)?;
    let collection = db.collection::<Storage>("local");

    let filter = doc! { "key": "local" };

    if let Some(storage) = collection.find_one(filter.clone())? {
        Ok(storage.credentials)
    } else {
        Err(Box::from(
            "Storage not found when retrieving credentials...",
        ))
    }
}

pub fn retrieve_cves() -> Result<Vec<CVE>, Box<dyn std::error::Error>> {
    let db_path = String::from("./cache_storage/local");
    let db = Database::open_file(db_path)?;
    let collection = db.collection::<Storage>("local");

    let filter = doc! { "key": "local" };

    if let Some(storage) = collection.find_one(filter.clone())? {
        Ok(storage.cves)
    } else {
        Err(Box::from("Storage not found when retrieving cves..."))
    }
}

pub fn retrieve_assets() -> Result<Vec<Asset>, Box<dyn std::error::Error>> {
    let db_path = String::from("./cache_storage/local");
    let db = Database::open_file(db_path)?;
    let collection = db.collection::<Storage>("local");

    let filter = doc! { "key": "local" };

    if let Some(storage) = collection.find_one(filter)? {
        Ok(storage.assets)
    } else {
        Err(Box::from("Storage not found when retrieving assets..."))
    }
}

pub fn retrieve_connections() -> Result<Vec<Connection>, Box<dyn std::error::Error>> {
    let db_path = String::from("./cache_storage/local");
    let db = Database::open_file(db_path)?;
    let collection = db.collection::<Storage>("local");

    let filter = doc! { "key": "local" };

    if let Some(storage) = collection.find_one(filter)? {
        Ok(storage.connections)
    } else {
        Err(Box::from("Storage not found when retrieving connections..."))
    }
}

// Set methods for credentials, cves, assets
pub fn update_credentials(credentials: Credentials) -> Result<(), Box<dyn Error>> {
    let db_path = String::from("./cache_storage/local");

    let db = Database::open_file(db_path).map_err(|e| {
        eprintln!("Failed to open database: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    let collection = db.collection::<Storage>("local");

    let credentials_bson = to_bson(&credentials).map_err(|e| {
        eprintln!("Failed to convert credentials to BSON: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    let update_result = collection.update_one(
        doc! { "key": "local" },
        doc! { "$set": { "credentials": credentials_bson } },
    ).map_err(|e| {
        eprintln!("Failed to update credentials: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    if update_result.modified_count == 0 {
        eprintln!("No document matched the filter for updating credentials.");
    } else {
        println!("Credentials updated successfully.");
    }

    Ok(())
}

pub fn update_cves(cves: Vec<CVE>) -> Result<(), Box<dyn Error>> {
    let db_path = String::from("./cache_storage/local");

    let db = Database::open_file(db_path).map_err(|e| {
        eprintln!("Failed to open database: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    let collection = db.collection::<Storage>("local");

    let cves_bson = to_bson(&cves).map_err(|e| {
        eprintln!("Failed to convert cves to BSON: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    collection.update_one(
        doc! { "key": "local" },
        doc! { "$set": { "cves": cves_bson } },
    ).map_err(|e| {
        eprintln!("Failed to update cves: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    Ok(())
}

pub fn update_assets(assets: Vec<Asset>) -> Result<(), Box<dyn Error>> {
    let db_path = String::from("./cache_storage/local");

    let db = Database::open_file(db_path).map_err(|e| {
        eprintln!("Failed to open database: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    let collection = db.collection::<Storage>("local");

    let assets_bson = to_bson(&assets).map_err(|e| {
        eprintln!("Failed to convert assets to BSON: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    collection.update_one(
        doc! { "key": "local" },
        doc! { "$set": { "assets": assets_bson } },
    ).map_err(|e| {
        eprintln!("Failed to update assets: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    Ok(())
}

pub fn update_connections(connections: Vec<Connection>) -> Result<(), Box<dyn Error>> {
    let db_path = String::from("./cache_storage/local");

    let db = Database::open_file(db_path).map_err(|e| {
        eprintln!("Failed to open database: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    let collection = db.collection::<Storage>("local");

    let connections_bson = to_bson(&connections).map_err(|e| {
        eprintln!("Failed to convert connections to BSON: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    collection.update_one(
        doc! { "key": "local" },
        doc! { "$set": { "connections": connections_bson } },
    ).map_err(|e| {
        eprintln!("Failed to update connections: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    Ok(())
}
