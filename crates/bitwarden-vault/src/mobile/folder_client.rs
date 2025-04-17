use bitwarden_core::Client;

use crate::{
    error::{DecryptError, EncryptError},
    Folder, FolderView, VaultClient,
};

pub struct ClientFolders {
    pub(crate) client: Client,
}

impl ClientFolders {
    pub fn encrypt(&self, folder_view: FolderView) -> Result<Folder, EncryptError> {
        let key_store = self.client.internal.get_key_store();
        let folder = key_store.encrypt(folder_view)?;
        Ok(folder)
    }

    pub fn decrypt(&self, folder: Folder) -> Result<FolderView, DecryptError> {
        let key_store = self.client.internal.get_key_store();
        let folder_view = key_store.decrypt(&folder)?;
        Ok(folder_view)
    }

    pub fn decrypt_list(&self, folders: Vec<Folder>) -> Result<Vec<FolderView>, DecryptError> {
        let key_store = self.client.internal.get_key_store();
        let views = key_store.decrypt_list(&folders)?;
        Ok(views)
    }
}

impl VaultClient {
    pub fn folders(&self) -> ClientFolders {
        ClientFolders {
            client: self.client.clone(),
        }
    }
}
