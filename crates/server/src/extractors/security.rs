use crate::error::{ServerError, ServerError::*};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{errors::ErrorKind, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub exp: u32,
    pub address: String,
}

pub struct Auth(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Unauthorized("Missing Authorization".to_owned()))
            .and_then(|bearer| decode_token(bearer.token()))
            .map(Self)
    }
}

fn decode_token(token: &str) -> Result<Claims, ServerError> {
    let access_secret = shared::read_env("JWT_SECRET")?;

    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(access_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|err| match err.kind() {
        ErrorKind::ExpiredSignature => Unauthorized("Expired token".to_owned()),
        _ => Unauthorized("Invalid token".to_owned()),
    })
    .map(|token_data| token_data.claims)
}
