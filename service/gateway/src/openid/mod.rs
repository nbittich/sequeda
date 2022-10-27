mod auth_redirect;
mod auth_request;
mod client;
mod router;

pub use router::open_id_router;


use openidconnect::core::{
    CoreAuthDisplay, CoreAuthPrompt, CoreGenderClaim, CoreJsonWebKey, CoreJsonWebKeyType,
    CoreJsonWebKeyUse, CoreJweContentEncryptionAlgorithm, CoreJwsSigningAlgorithm,
};
use openidconnect::{
    Client, EmptyAdditionalClaims, EmptyExtraTokenFields, IdTokenFields,
    RevocationErrorResponseType, StandardErrorResponse, StandardTokenIntrospectionResponse,
    StandardTokenResponse,
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
