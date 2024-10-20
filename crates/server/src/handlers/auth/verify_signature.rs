use std::str::FromStr;

use crate::{
    error::ServerError::{self, *},
    extractors::{
        security::Claims,
        states::{Postgres, Redis},
    },
};
use alloy::{primitives::Address, signers::Signature};
use axum::Json;
use chrono::{Duration, Utc};
use database::repositories;
use deadpool_redis::redis::AsyncCommands;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use shared::user_sign_message_key;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct VerifySignaturePayload {
    address: String,
    message: String,
    signature: String,
}

#[utoipa::path(
    tag = "Auth",
    post,
    path = "/api/auth/verify-signature",
    request_body = VerifySignaturePayload,
    responses(
        (status = 200, body = VerifySignatureResponse)
    )
)]
pub async fn verify_signature(
    Postgres(db): Postgres,
    Redis(mut redis): Redis,
    Json(VerifySignaturePayload {
        address,
        message,
        signature,
    }): Json<VerifySignaturePayload>,
) -> Result<Json<VerifySignatureResponse>, ServerError> {
    let address =
        Address::from_str(&address).map_err(|_| Unauthorized("invalid address".to_string()))?;

    let stored_message: String = redis.get(user_sign_message_key(&address)).await?;

    if stored_message != message {
        return Err(Unauthorized("wrong message".to_string()));
    };

    let recovered_address = Signature::from_str(&signature)
        .and_then(|signature| signature.recover_address_from_msg(message))
        .map_err(|_| Unauthorized("wrong signature, message not matched".to_string()))?;

    if recovered_address != address {
        return Err(Unauthorized(
            "wrong signature, address not matched".to_string(),
        ));
    };

    repositories::user::create_if_not_exist(&db, address.to_string()).await?;

    let secret = shared::read_env("JWT_SECRET")?;

    let header = Header::new(Algorithm::HS256);

    let secret_key = EncodingKey::from_secret(secret.as_bytes());

    let access_exp = Utc::now()
        .checked_add_signed(Duration::days(3))
        .ok_or(Custom(
            "get none from checked_add_signed() duration".to_string(),
        ))?
        .timestamp() as u32;

    let claims = Claims {
        exp: access_exp,
        address: address.to_string(),
    };

    let access_token =
        jsonwebtoken::encode(&header, &claims, &secret_key).map_err(|e| Custom(e.to_string()))?;

    let response = VerifySignatureResponse { access_token };

    Ok(Json(response))
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VerifySignatureResponse {
    access_token: String,
}
