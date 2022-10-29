mod auth_redirect;
mod auth_request;
mod client;
mod router;




pub use router::open_id_router;

use openidconnect::core::{
    CoreAuthDisplay, CoreAuthPrompt, CoreClaimName, CoreClaimType, CoreClientAuthMethod,
    CoreGenderClaim, CoreGrantType, CoreJsonWebKey, CoreJsonWebKeyType, CoreJsonWebKeyUse,
    CoreJweContentEncryptionAlgorithm, CoreJweKeyManagementAlgorithm, CoreJwsSigningAlgorithm,
    CoreResponseMode, CoreResponseType, CoreSubjectIdentifierType,
};
use openidconnect::{
    AdditionalClaims, AdditionalProviderMetadata, Client, EmptyExtraTokenFields, IdTokenClaims,
    IdTokenFields, ProviderMetadata, RevocationErrorResponseType, StandardErrorResponse,
    StandardTokenIntrospectionResponse, StandardTokenResponse,
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
pub struct AllOtherClaims {
    realm_access: RealmAccess,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RealmAccess {
    roles: Vec<String>,
}
impl AdditionalClaims for AllOtherClaims {}

pub type CustomIdTokenClaims = IdTokenClaims<AllOtherClaims, CoreGenderClaim>;
