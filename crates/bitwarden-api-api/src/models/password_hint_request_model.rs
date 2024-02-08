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
pub struct PasswordHintRequestModel {
    #[serde(rename = "email")]
    pub email: String,
}

impl PasswordHintRequestModel {
    pub fn new(email: String) -> PasswordHintRequestModel {
        PasswordHintRequestModel { email }
    }
}
