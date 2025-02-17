use std::sync::Arc;

use bitwarden_vault::{Collection, CollectionView, VaultClientExt};

use crate::{error::Error, Client, Result};

#[derive(uniffi::Object)]
pub struct ClientCollections(pub Arc<Client>);

#[uniffi::export]
impl ClientCollections {
    /// Decrypt collection
    pub fn decrypt(&self, collection: Collection) -> Result<CollectionView> {
        Ok(self
            .0
             .0
            .vault()
            .collections()
            .decrypt(collection)
            .map_err(Error::Decrypt)?)
    }

    /// Decrypt collection list
    pub fn decrypt_list(&self, collections: Vec<Collection>) -> Result<Vec<CollectionView>> {
        Ok(self
            .0
             .0
            .vault()
            .collections()
            .decrypt_list(collections)
            .map_err(Error::Decrypt)?)
    }
}
