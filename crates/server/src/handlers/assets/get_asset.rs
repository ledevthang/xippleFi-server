use crate::{
    error::ServerError::{self, *},
    extractors::states::Postgres,
};
use axum::{extract::Path, Json};
use database::{models::AssetModel, repositories};
use serde::{Deserialize, Serialize};
use shared::constants::Asset;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct GetAssetParams {
    asset_id: Asset,
}

#[utoipa::path(
    tag = "Assets",
    get,
    path = "/assets/{asset_id}",
    params(GetAssetParams),
    responses(
        (status = 200, body = GetAssetResponse)
    )
)]
pub async fn get_asset(
    Path(GetAssetParams { asset_id }): Path<GetAssetParams>,
    Postgres(db): Postgres,
) -> Result<Json<GetAssetResponse>, ServerError> {
    let asset = repositories::asset::find_by_id(&db, &asset_id)
        .await?
        .ok_or(Internal(format!("not found asset with id {:#?}", asset_id)))?;

    let response = GetAssetResponse { asset };

    Ok(Json(response))
}

#[derive(Serialize, ToSchema)]
pub struct GetAssetResponse {
    asset: AssetModel,
}
