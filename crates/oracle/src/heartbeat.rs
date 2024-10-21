use crate::error::OracleError;
use alloy::primitives::U256;
use contracts::oracle::ORACLE_CONTRACTS;
use database::{repositories::asset, sea_orm::DatabaseConnection};
use futures_util::future::try_join_all;
use shared::constants::Asset;
use std::str::FromStr;

pub async fn heartbeat(db: &DatabaseConnection) -> Result<(), OracleError> {
    let assets = asset::find_if_timeline_reached(db).await?;

    if assets.is_empty() {
        return Ok(());
    }

    let tasks = assets.into_iter().map(|asset| async move {
        let price_dec = asset.real_time_price;
        let price = U256::from_str(&price_dec.to_string()).unwrap();

        let id = Asset::from_coincap_id(&asset.id);

        let contract = ORACLE_CONTRACTS.get(&id).expect("impossible");

        let hash = contract.write_asset(price).await?;
        asset::udate_price(db, &id, price_dec).await?;

        tracing::info!("updated price to contact {:#?} >> {}", id, hash);

        Ok::<(), OracleError>(())
    });

    try_join_all(tasks).await?;

    Ok(())
}
