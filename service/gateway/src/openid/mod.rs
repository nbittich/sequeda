mod auth_redirect;
mod client;

pub const OPENID_CLIENT_ID: &str = "OPENID_CLIENT_ID";
pub const OPENID_CLIENT_SECRET: &str = "OPENID_CLIENT_SECRET";
pub const OPENID_ISSUER_URL: &str = "OPENID_ISSUER_URL";
pub const OPENID_SCOPES: &str = "OPENID_SCOPES";
use openidconnect::core::{
    CoreAuthDisplay, CoreAuthPrompt, CoreClientAuthMethod, CoreGenderClaim, CoreGrantType,
    CoreJsonWebKey, CoreJsonWebKeyType, CoreJsonWebKeyUse, CoreJweContentEncryptionAlgorithm,
    CoreJweKeyManagementAlgorithm, CoreJwsSigningAlgorithm, CoreResponseMode, CoreResponseType,
    CoreSubjectIdentifierType,
};
use openidconnect::{
    AdditionalProviderMetadata, AuthenticationFlow, Client, CsrfToken, EmptyAdditionalClaims,
    EmptyExtraTokenFields, IdTokenFields, Nonce, ProviderMetadata, RedirectUrl,
    RevocationErrorResponseType, RevocationUrl, Scope, StandardErrorResponse,
    StandardTokenIntrospectionResponse, StandardTokenResponse,
};
type RawOpenIdClient = Client<
    EmptyAdditionalClaims,
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
            EmptyAdditionalClaims,
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
