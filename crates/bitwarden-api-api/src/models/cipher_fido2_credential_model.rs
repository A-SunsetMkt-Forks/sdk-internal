/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CipherFido2CredentialModel {
    #[serde(rename = "credentialId", skip_serializing_if = "Option::is_none")]
    pub credential_id: Option<String>,
    #[serde(rename = "keyType", skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    #[serde(rename = "keyAlgorithm", skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,
    #[serde(rename = "keyCurve", skip_serializing_if = "Option::is_none")]
    pub key_curve: Option<String>,
    #[serde(rename = "keyValue", skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
    #[serde(rename = "rpId", skip_serializing_if = "Option::is_none")]
    pub rp_id: Option<String>,
    #[serde(rename = "rpName", skip_serializing_if = "Option::is_none")]
    pub rp_name: Option<String>,
    #[serde(rename = "userHandle", skip_serializing_if = "Option::is_none")]
    pub user_handle: Option<String>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "userDisplayName", skip_serializing_if = "Option::is_none")]
    pub user_display_name: Option<String>,
    #[serde(rename = "counter", skip_serializing_if = "Option::is_none")]
    pub counter: Option<String>,
    #[serde(rename = "discoverable", skip_serializing_if = "Option::is_none")]
    pub discoverable: Option<String>,
    #[serde(rename = "creationDate")]
    pub creation_date: String,
}

impl CipherFido2CredentialModel {
    pub fn new(creation_date: String) -> CipherFido2CredentialModel {
        CipherFido2CredentialModel {
            credential_id: None,
            key_type: None,
            key_algorithm: None,
            key_curve: None,
            key_value: None,
            rp_id: None,
            rp_name: None,
            user_handle: None,
            user_name: None,
            user_display_name: None,
            counter: None,
            discoverable: None,
            creation_date,
        }
    }
}
