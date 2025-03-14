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
pub struct TaxInformationRequestBody {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    #[serde(rename = "taxId", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(rename = "taxIdType", skip_serializing_if = "Option::is_none")]
    pub tax_id_type: Option<String>,
    #[serde(rename = "line1", skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[serde(rename = "line2", skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl TaxInformationRequestBody {
    pub fn new(country: String, postal_code: String) -> TaxInformationRequestBody {
        TaxInformationRequestBody {
            country,
            postal_code,
            tax_id: None,
            tax_id_type: None,
            line1: None,
            line2: None,
            city: None,
            state: None,
        }
    }
}
