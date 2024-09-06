/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateTdeOffboardingPasswordRequestModel {
    #[serde(rename = "newMasterPasswordHash")]
    pub new_master_password_hash: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "masterPasswordHint", skip_serializing_if = "Option::is_none")]
    pub master_password_hint: Option<String>,
}

impl UpdateTdeOffboardingPasswordRequestModel {
    pub fn new(
        new_master_password_hash: String,
        key: String,
    ) -> UpdateTdeOffboardingPasswordRequestModel {
        UpdateTdeOffboardingPasswordRequestModel {
            new_master_password_hash,
            key,
            master_password_hint: None,
        }
    }
}