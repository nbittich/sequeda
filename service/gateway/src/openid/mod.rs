mod auth_redirect;
mod auth_request;
mod client;
mod router;

use std::collections::HashMap;

use async_session::serde_json;
pub use router::open_id_router;

use openidconnect::core::{
    CoreAuthDisplay, CoreAuthPrompt, CoreGenderClaim, CoreJsonWebKey, CoreJsonWebKeyType,
    CoreJsonWebKeyUse, CoreJweContentEncryptionAlgorithm, CoreJwsSigningAlgorithm, CoreClientAuthMethod, CoreClaimName, CoreClaimType, CoreGrantType, CoreJweKeyManagementAlgorithm, CoreResponseMode, CoreResponseType, CoreSubjectIdentifierType,
};
use openidconnect::{
    Client, EmptyAdditionalClaims, EmptyExtraTokenFields, IdTokenFields,
    RevocationErrorResponseType, StandardErrorResponse, StandardTokenIntrospectionResponse,
    StandardTokenResponse, ProviderMetadata, AdditionalClaims, AdditionalProviderMetadata, IdTokenClaims,
};
use serde::{Deserialize, Serialize};
type RawOpenIdClient = Client<
    AllOtherClaims,
    CoreAuthDisplay,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJwsSigningAlgorithm,
    CoreJsonWebKeyType,
    CoreJsonWebKeyUse,
    CoreJsonWebKey,
    CoreAuthPrompt,
    StandardErrorResponse<openidconnect::core::CoreErrorResponseType>,
    StandardTokenResponse<
        IdTokenFields<
            AllOtherClaims,
            EmptyExtraTokenFields,
            CoreGenderClaim,
            CoreJweContentEncryptionAlgorithm,
            CoreJwsSigningAlgorithm,
            CoreJsonWebKeyType,
        >,
        openidconnect::core::CoreTokenType,
    >,
    openidconnect::core::CoreTokenType,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, openidconnect::core::CoreTokenType>,
    openidconnect::core::CoreRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
>;
type OpenIdProviderMetadata = ProviderMetadata<
    RevocationEndpointProviderMetadata,
    CoreAuthDisplay,
    CoreClientAuthMethod,
    CoreClaimName,
    CoreClaimType,
    CoreGrantType,
    CoreJweContentEncryptionAlgorithm,
    CoreJweKeyManagementAlgorithm,
    CoreJwsSigningAlgorithm,
    CoreJsonWebKeyType,
    CoreJsonWebKeyUse,
    CoreJsonWebKey,
    CoreResponseMode,
    CoreResponseType,
    CoreSubjectIdentifierType,
>;


#[derive(Clone, Debug, Deserialize, Serialize)]
struct RevocationEndpointProviderMetadata {
    revocation_endpoint: String,
}

impl AdditionalProviderMetadata for RevocationEndpointProviderMetadata {}


#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AllOtherClaims(pub HashMap<String, serde_json::Value>);
impl AdditionalClaims for AllOtherClaims {}

pub type CustomIdTokenClaims = IdTokenClaims<AllOtherClaims, CoreGenderClaim>;
