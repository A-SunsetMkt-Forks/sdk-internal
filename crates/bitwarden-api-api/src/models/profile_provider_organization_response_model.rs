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
pub struct ProfileProviderOrganizationResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "usePolicies", skip_serializing_if = "Option::is_none")]
    pub use_policies: Option<bool>,
    #[serde(rename = "useSso", skip_serializing_if = "Option::is_none")]
    pub use_sso: Option<bool>,
    #[serde(rename = "useKeyConnector", skip_serializing_if = "Option::is_none")]
    pub use_key_connector: Option<bool>,
    #[serde(rename = "useScim", skip_serializing_if = "Option::is_none")]
    pub use_scim: Option<bool>,
    #[serde(rename = "useGroups", skip_serializing_if = "Option::is_none")]
    pub use_groups: Option<bool>,
    #[serde(rename = "useDirectory", skip_serializing_if = "Option::is_none")]
    pub use_directory: Option<bool>,
    #[serde(rename = "useEvents", skip_serializing_if = "Option::is_none")]
    pub use_events: Option<bool>,
    #[serde(rename = "useTotp", skip_serializing_if = "Option::is_none")]
    pub use_totp: Option<bool>,
    #[serde(rename = "use2fa", skip_serializing_if = "Option::is_none")]
    pub use2fa: Option<bool>,
    #[serde(rename = "useApi", skip_serializing_if = "Option::is_none")]
    pub use_api: Option<bool>,
    #[serde(rename = "useResetPassword", skip_serializing_if = "Option::is_none")]
    pub use_reset_password: Option<bool>,
    #[serde(rename = "useSecretsManager", skip_serializing_if = "Option::is_none")]
    pub use_secrets_manager: Option<bool>,
    #[serde(rename = "usePasswordManager", skip_serializing_if = "Option::is_none")]
    pub use_password_manager: Option<bool>,
    #[serde(rename = "usersGetPremium", skip_serializing_if = "Option::is_none")]
    pub users_get_premium: Option<bool>,
    #[serde(
        rename = "useCustomPermissions",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_custom_permissions: Option<bool>,
    #[serde(
        rename = "useActivateAutofillPolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_activate_autofill_policy: Option<bool>,
    #[serde(rename = "selfHost", skip_serializing_if = "Option::is_none")]
    pub self_host: Option<bool>,
    #[serde(rename = "seats", skip_serializing_if = "Option::is_none")]
    pub seats: Option<i32>,
    #[serde(rename = "maxCollections", skip_serializing_if = "Option::is_none")]
    pub max_collections: Option<i32>,
    #[serde(rename = "maxStorageGb", skip_serializing_if = "Option::is_none")]
    pub max_storage_gb: Option<i32>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::OrganizationUserStatusType>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::OrganizationUserType>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "ssoBound", skip_serializing_if = "Option::is_none")]
    pub sso_bound: Option<bool>,
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<models::Permissions>>,
    #[serde(
        rename = "resetPasswordEnrolled",
        skip_serializing_if = "Option::is_none"
    )]
    pub reset_password_enrolled: Option<bool>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    #[serde(rename = "organizationUserId", skip_serializing_if = "Option::is_none")]
    pub organization_user_id: Option<uuid::Uuid>,
    #[serde(
        rename = "hasPublicAndPrivateKeys",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_public_and_private_keys: Option<bool>,
    #[serde(rename = "providerId", skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<uuid::Uuid>,
    #[serde(rename = "providerName", skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(rename = "providerType", skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<models::ProviderType>,
    #[serde(
        rename = "familySponsorshipFriendlyName",
        skip_serializing_if = "Option::is_none"
    )]
    pub family_sponsorship_friendly_name: Option<String>,
    #[serde(
        rename = "familySponsorshipAvailable",
        skip_serializing_if = "Option::is_none"
    )]
    pub family_sponsorship_available: Option<bool>,
    #[serde(rename = "productTierType", skip_serializing_if = "Option::is_none")]
    pub product_tier_type: Option<models::ProductTierType>,
    #[serde(
        rename = "keyConnectorEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub key_connector_enabled: Option<bool>,
    #[serde(rename = "keyConnectorUrl", skip_serializing_if = "Option::is_none")]
    pub key_connector_url: Option<String>,
    #[serde(
        rename = "familySponsorshipLastSyncDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub family_sponsorship_last_sync_date: Option<String>,
    #[serde(
        rename = "familySponsorshipValidUntil",
        skip_serializing_if = "Option::is_none"
    )]
    pub family_sponsorship_valid_until: Option<String>,
    #[serde(
        rename = "familySponsorshipToDelete",
        skip_serializing_if = "Option::is_none"
    )]
    pub family_sponsorship_to_delete: Option<bool>,
    #[serde(
        rename = "accessSecretsManager",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_secrets_manager: Option<bool>,
    #[serde(
        rename = "limitCollectionCreation",
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_collection_creation: Option<bool>,
    #[serde(
        rename = "limitCollectionDeletion",
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_collection_deletion: Option<bool>,
    #[serde(rename = "limitItemDeletion", skip_serializing_if = "Option::is_none")]
    pub limit_item_deletion: Option<bool>,
    #[serde(
        rename = "allowAdminAccessToAllCollectionItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_admin_access_to_all_collection_items: Option<bool>,
    /// Indicates if the organization manages the user.
    #[serde(
        rename = "userIsManagedByOrganization",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_is_managed_by_organization: Option<bool>,
    #[serde(rename = "useRiskInsights", skip_serializing_if = "Option::is_none")]
    pub use_risk_insights: Option<bool>,
}

impl ProfileProviderOrganizationResponseModel {
    pub fn new() -> ProfileProviderOrganizationResponseModel {
        ProfileProviderOrganizationResponseModel {
            object: None,
            id: None,
            name: None,
            use_policies: None,
            use_sso: None,
            use_key_connector: None,
            use_scim: None,
            use_groups: None,
            use_directory: None,
            use_events: None,
            use_totp: None,
            use2fa: None,
            use_api: None,
            use_reset_password: None,
            use_secrets_manager: None,
            use_password_manager: None,
            users_get_premium: None,
            use_custom_permissions: None,
            use_activate_autofill_policy: None,
            self_host: None,
            seats: None,
            max_collections: None,
            max_storage_gb: None,
            key: None,
            status: None,
            r#type: None,
            enabled: None,
            sso_bound: None,
            identifier: None,
            permissions: None,
            reset_password_enrolled: None,
            user_id: None,
            organization_user_id: None,
            has_public_and_private_keys: None,
            provider_id: None,
            provider_name: None,
            provider_type: None,
            family_sponsorship_friendly_name: None,
            family_sponsorship_available: None,
            product_tier_type: None,
            key_connector_enabled: None,
            key_connector_url: None,
            family_sponsorship_last_sync_date: None,
            family_sponsorship_valid_until: None,
            family_sponsorship_to_delete: None,
            access_secrets_manager: None,
            limit_collection_creation: None,
            limit_collection_deletion: None,
            limit_item_deletion: None,
            allow_admin_access_to_all_collection_items: None,
            user_is_managed_by_organization: None,
            use_risk_insights: None,
        }
    }
}
