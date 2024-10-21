use crate::{
    error::ServerError::{self},
    extractors::states::Postgres,
};
use axum::Json;
use database::{models::AssetModel, repositories};
use serde::Serialize;
use utoipa::ToSchema;

#[utoipa::path(
    tag = "Assets",
    get,
    path = "/assets",
    responses(
        (status = 200, body = GetAssetsResponse)
    )
)]
pub async fn get_assets(Postgres(db): Postgres) -> Result<Json<GetAssetsResponse>, ServerError> {
    let assets = repositories::asset::find_all(&db).await?;

    let response = GetAssetsResponse { assets };

    Ok(Json(response))
}

#[derive(Serialize, ToSchema)]
pub struct GetAssetsResponse {
    assets: Vec<AssetModel>,
}
