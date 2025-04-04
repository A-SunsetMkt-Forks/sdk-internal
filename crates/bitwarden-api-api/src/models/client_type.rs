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

///
#[repr(i64)]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
)]
pub enum ClientType {
    All = 0,
    Web = 1,
    Browser = 2,
    Desktop = 3,
    Mobile = 4,
}

impl std::fmt::Display for ClientType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "0"),
            Self::Web => write!(f, "1"),
            Self::Browser => write!(f, "2"),
            Self::Desktop => write!(f, "3"),
            Self::Mobile => write!(f, "4"),
        }
    }
}

impl Default for ClientType {
    fn default() -> ClientType {
        Self::All
    }
}
