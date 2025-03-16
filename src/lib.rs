pub mod configuration {
    pub mod cve {
        pub mod endpoints {
            pub mod cve_endpoint;
            pub mod nist_endpoint;
        }

        pub mod importers {
            pub mod cve_importer;
            pub mod nessus_importer;
        }
    }
    pub mod assets {
        pub mod assets_config;
    }
}

pub mod analysis {
    pub mod ranking;
}

pub mod storage {
    pub mod db_handler;
}

pub mod schema {
    pub mod cve;
    pub mod rank;
    pub mod asset;
    pub mod storage;
}
