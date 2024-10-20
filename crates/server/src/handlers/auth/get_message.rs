use std::str::FromStr;

use crate::{
    error::ServerError::{self, *},
    extractors::states::Redis,
};
use alloy::primitives::{eip191_hash_message, Address};
use axum::{extract::Query, Json};
use deadpool_redis::redis::{AsyncCommands, SetExpiry, SetOptions};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use shared::user_sign_message_key;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetMessageQuery {
    address: String,
}

#[utoipa::path(
    tag = "Auth",
    get,
    path = "/api/auth/message",
    params(GetMessageQuery),
    responses(
        (status = 200, body = GetMessageResponse)
    )
)]
pub async fn get_message(
    Query(GetMessageQuery { address }): Query<GetMessageQuery>,
    Redis(mut redis): Redis,
) -> Result<Json<GetMessageResponse>, ServerError> {
    let address =
        Address::from_str(&address).map_err(|_| BadRequest("invalid address".to_string()))?;

    let message: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(13)
        .map(char::from)
        .collect();

    let message = eip191_hash_message(message).to_string();

    let set_option = SetOptions::default().with_expiration(SetExpiry::EX(300));

    redis
        .set_options(user_sign_message_key(&address), &message, set_option) //5mins
        .await?;

    let response = GetMessageResponse { message };

    Ok(Json(response))
}

#[derive(Serialize, ToSchema)]
pub struct GetMessageResponse {
    message: String,
}
