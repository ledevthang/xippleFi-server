mod error;

use alloy::{
    eips::BlockNumberOrTag,
    primitives::address,
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::{Filter, FilterBlockOption},
    transports::http::{reqwest::Url, Client, Http},
};
use database::{
    repositories::setting::{self, Config},
    sea_orm::{ConnectOptions, Database, DatabaseConnection},
};
use error::ScannerError;
use std::{str::FromStr, time::Duration};
use tracing_subscriber::EnvFilter;

type BlockNumber = u64;
const DISTANCE: u64 = 10000;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("failed to load env");

    let filter = EnvFilter::builder()
        .from_env()
        .unwrap()
        .add_directive("scanner=debug".parse().unwrap());

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false);

    let db = Database::connect(opt)
        .await
        .expect("can not connect to database");

    let rpc_url = Url::from_str("https://rpc-evm-sidechain.xrpl.org").expect("invalid rpc url");

    let provider = ProviderBuilder::new().on_http(rpc_url);

    let key = Config::LatestScannedBlock;

    let mut latest_scanned = {
        let config = setting::find(&db, &key)
            .await
            .expect("failed to find latest_scanned_block")
            .expect("missing latest_scanned_block setting");

        u64::from_str(&config).expect("invalid latest_scanned_block, u64 expected")
    };

    let mut filter = Filter::new()
        .address(address!("95E0e5f14Edd1a28ada89b0F686eAaF81Da91c37"))
        .from_block(latest_scanned)
        .to_block(latest_scanned + DISTANCE);

    loop {
        let latest_scanned_block =
            scan(&provider, &db, &mut filter)
                .await
                .unwrap_or_else(|error| {
                    tracing::error!("scan error {:#?}", error);
                    latest_scanned
                });

        latest_scanned = latest_scanned_block;

        setting::set(&db, &key, latest_scanned_block.to_string())
            .await
            .expect("failed to set latest_scanned_block");

        tracing::info!("scanned to {}", latest_scanned_block);

        tokio::time::sleep(Duration::from_secs(20)).await;
    }
}

async fn scan(
    provider: &RootProvider<Http<Client>>,
    _db: &DatabaseConnection,
    filter: &mut Filter,
) -> Result<BlockNumber, ScannerError> {
    let mut to_block = filter.get_to_block().unwrap();

    let latest_block = provider.get_block_number().await?;

    if to_block > latest_block {
        filter.block_option = filter
            .block_option
            .with_to_block(BlockNumberOrTag::Number(latest_block));

        to_block = latest_block;
    };

    let logs = provider.get_logs(&filter).await?;

    dbg!(logs.len());

    for _log in logs {}

    let next = to_block + 1;

    filter.block_option = FilterBlockOption::Range {
        from_block: BlockNumberOrTag::Number(next).into(),
        to_block: BlockNumberOrTag::Number(next + DISTANCE).into(),
    };

    Ok(to_block)
}
