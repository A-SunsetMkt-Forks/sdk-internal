use std::sync::Arc;

use bitwarden::mobile::crypto::{InitOrgCryptoRequest, InitUserCryptoRequest};

use crate::{error::Result, Client};

#[derive(uniffi::Object)]
pub struct ClientCrypto(pub(crate) Arc<Client>);

#[uniffi::export]
impl ClientCrypto {
    /// Initialization method for the user crypto. Needs to be called before any other crypto operations.
    pub async fn initialize_user_crypto(&self, req: InitUserCryptoRequest) -> Result<()> {
        Ok(self
            .0
             .0
            .write()
            .await
            .crypto()
            .initialize_user_crypto(req)
            .await?)
    }

    /// Initialization method for the organization crypto. Needs to be called after `initialize_user_crypto` but before any other crypto operations.
    pub async fn initialize_org_crypto(&self, req: InitOrgCryptoRequest) -> Result<()> {
        Ok(self
            .0
             .0
            .write()
            .await
            .crypto()
            .initialize_org_crypto(req)
            .await?)
    }
}
