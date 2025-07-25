use std::fmt::{Display, Formatter};

use bitwarden_exporters::ExportError;
use bitwarden_generators::{PassphraseError, PasswordError, UsernameError};

// Name is converted from *Error to *Exception, so we can't just name the enum Error because
// Exception already exists
#[derive(uniffi::Error, Debug)]
#[uniffi(flat_error)]
pub enum BitwardenError {
    E(Error),
}

impl From<Error> for BitwardenError {
    fn from(e: Error) -> Self {
        Self::E(e)
    }
}

impl Display for BitwardenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::E(e) => Display::fmt(e, f),
        }
    }
}

impl std::error::Error for BitwardenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BitwardenError::E(e) => Some(e),
        }
    }
}

pub type Result<T, E = BitwardenError> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Api(#[from] bitwarden_core::ApiError),
    #[error(transparent)]
    DeriveKeyConnector(#[from] bitwarden_core::key_management::crypto::DeriveKeyConnectorError),
    #[error(transparent)]
    EncryptionSettings(
        #[from] bitwarden_core::client::encryption_settings::EncryptionSettingsError,
    ),
    #[error(transparent)]
    EnrollAdminPasswordReset(
        #[from] bitwarden_core::key_management::crypto::EnrollAdminPasswordResetError,
    ),
    #[error(transparent)]
    MobileCrypto(#[from] bitwarden_core::key_management::crypto::CryptoClientError),
    #[error(transparent)]
    AuthValidate(#[from] bitwarden_core::auth::AuthValidateError),
    #[error(transparent)]
    ApproveAuthRequest(#[from] bitwarden_core::auth::ApproveAuthRequestError),
    #[error(transparent)]
    TrustDevice(#[from] bitwarden_core::auth::auth_client::TrustDeviceError),

    #[error(transparent)]
    Fingerprint(#[from] bitwarden_core::platform::FingerprintError),
    #[error(transparent)]
    UserFingerprint(#[from] bitwarden_core::platform::UserFingerprintError),

    #[error(transparent)]
    Crypto(#[from] bitwarden_crypto::CryptoError),

    // Generators
    #[error(transparent)]
    Username(#[from] UsernameError),
    #[error(transparent)]
    Passphrase(#[from] PassphraseError),
    #[error(transparent)]
    Password(#[from] PasswordError),

    // Vault
    #[error(transparent)]
    Cipher(#[from] bitwarden_vault::CipherError),
    #[error(transparent)]
    Totp(#[from] bitwarden_vault::TotpError),
    #[error(transparent)]
    Decrypt(#[from] bitwarden_vault::DecryptError),
    #[error(transparent)]
    DecryptFile(#[from] bitwarden_vault::DecryptFileError),
    #[error(transparent)]
    Encrypt(#[from] bitwarden_vault::EncryptError),
    #[error(transparent)]
    EncryptFile(#[from] bitwarden_vault::EncryptFileError),

    // Send
    #[error(transparent)]
    SendDecrypt(#[from] bitwarden_send::SendDecryptError),
    #[error(transparent)]
    SendDecryptFile(#[from] bitwarden_send::SendDecryptFileError),
    #[error(transparent)]
    SendEncrypt(#[from] bitwarden_send::SendEncryptError),
    #[error(transparent)]
    SendEncryptFile(#[from] bitwarden_send::SendEncryptFileError),

    #[error(transparent)]
    Export(#[from] ExportError),

    // Fido
    #[error(transparent)]
    MakeCredential(#[from] bitwarden_fido::MakeCredentialError),
    #[error(transparent)]
    GetAssertion(#[from] bitwarden_fido::GetAssertionError),
    #[error(transparent)]
    SilentlyDiscoverCredentials(#[from] bitwarden_fido::SilentlyDiscoverCredentialsError),
    #[error(transparent)]
    CredentialsForAutofill(#[from] bitwarden_fido::CredentialsForAutofillError),
    #[error(transparent)]
    DecryptFido2AutofillCredentials(#[from] bitwarden_fido::DecryptFido2AutofillCredentialsError),
    #[error(transparent)]
    Fido2Client(#[from] bitwarden_fido::Fido2ClientError),

    #[error(transparent)]
    SshGeneration(#[from] bitwarden_ssh::error::KeyGenerationError),
    #[error(transparent)]
    SshImport(#[from] bitwarden_ssh::error::SshKeyImportError),
}
