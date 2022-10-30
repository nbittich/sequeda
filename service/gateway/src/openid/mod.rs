mod auth_redirect;
mod auth_request;
mod client;
mod router;
mod user;

use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
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
type CustomTokenResponse = StandardTokenResponse<
    IdTokenFields<
        AllOtherClaims,
        EmptyExtraTokenFields,
        CoreGenderClaim,
        CoreJweContentEncryptionAlgorithm,
        CoreJwsSigningAlgorithm,
        CoreJsonWebKeyType,
    >,
    openidconnect::core::CoreTokenType,
>;
use serde::{Deserialize, Serialize};

use self::auth_redirect::LoginPageRedirect;
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
    CustomTokenResponse,
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
    groups: Vec<String>,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RealmAccess {
    roles: Vec<String>,
}
impl AdditionalClaims for AllOtherClaims {}

pub type CustomIdTokenClaims = IdTokenClaims<AllOtherClaims, CoreGenderClaim>;

pub async fn destroy_session(store: &RedisSessionStore, session: Session) -> LoginPageRedirect {
    if let Err(e) = store.destroy_session(session).await {
        tracing::error!("{e}");
    }
    LoginPageRedirect
}
