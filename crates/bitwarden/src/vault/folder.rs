use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{CipherString, Decryptable, Encryptable},
    error::Result,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    id: Uuid,
    name: CipherString,
    revision_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FolderView {
    id: Uuid,
    name: String,
    revision_date: DateTime<Utc>,
}

impl Encryptable<Folder> for FolderView {
    fn encrypt(self, enc: &EncryptionSettings, _org: &Option<Uuid>) -> Result<Folder> {
        Ok(Folder {
            id: self.id,
            name: self.name.encrypt(enc, &None)?,
            revision_date: self.revision_date,
        })
    }
}

impl Decryptable<FolderView> for Folder {
    fn decrypt(&self, enc: &EncryptionSettings, _org: &Option<Uuid>) -> Result<FolderView> {
        Ok(FolderView {
            id: self.id,
            name: self.name.decrypt(enc, &None)?,
            revision_date: self.revision_date,
        })
    }
}