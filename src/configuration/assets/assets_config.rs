use crate::schema::asset::Asset;
pub struct AssetsList {
    assets: Vec<Asset>,
}

impl AssetsList {
    // AssetsList which holds a vector list of assets when passed in
    pub fn create(assets: Vec<Asset>) -> Self {
        AssetsList {
            assets,
        }
    }

    pub fn retrieve_assets(&self) -> &Vec<Asset> {
        &self.assets
    }
}
