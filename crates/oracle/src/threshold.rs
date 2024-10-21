use crate::{error::OracleError, PriceStr};
use alloy::primitives::utils::parse_ether;
use contracts::oracle::ORACLE_CONTRACTS;
use database::repositories::asset;
use database::sea_orm::{prelude::Decimal, DatabaseConnection};
use futures_util::future::try_join_all;
use shared::constants::Asset;
use std::str::FromStr;

pub async fn threshold(
    db: &DatabaseConnection,
    assets: impl IntoIterator<Item = (Asset, PriceStr)>,
) -> Result<(), OracleError> {
    let tasks = assets.into_iter().map(|(id, price)| async move {
        let price_in_wei = parse_ether(&price).map_err(|e| OracleError::Custom(e.to_string()))?;

        let price_wei_str = price_in_wei.to_string();

        let price =
            Decimal::from_str(&price_wei_str).map_err(|e| OracleError::Custom(e.to_string()))?;

        let Some(asset) = asset::find_by_id(db, &id).await? else {
            asset::create(db, &id, price).await?;

            return Ok(());
        };

        let asset_config = id.as_config();

        let change = (asset.price - price).abs() / price / Decimal::from(100);

        let threshold = Decimal::from_f32_retain(asset_config.threshhold).ok_or(
            OracleError::Custom("can not parse threshold config to decimal".to_string()),
        )?;

        if change < threshold {
            asset::update_realtime_price(db, &id, price).await?;
        } else {
            let contract = ORACLE_CONTRACTS.get(&Asset::BNB).expect("impossible");

            let hash = contract.write_asset(price_in_wei).await?;

            asset::udate_price(db, &id, price).await?;

            tracing::info!("updated price to contact {:#?} >> {}", id, hash);
        }

        Ok::<(), OracleError>(())
    });

    try_join_all(tasks).await?;

    Ok(())
}
