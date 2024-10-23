mod error;
mod event_handler;
use alloy::{
    eips::BlockNumberOrTag,
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::{BlockTransactionsKind, Filter, FilterBlockOption},
    sol_types::SolEventInterface,
    transports::http::{reqwest::Url, Client, Http},
};
use contracts::xipple_fi::XippleFi::XippleFiEvents;
use database::{
    repositories::{
        event,
        setting::{self, Config},
    },
    sea_orm::{sqlx::types::chrono::DateTime, ConnectOptions, Database, DatabaseConnection},
};
use error::ScannerError;
use event_handler::{handle_borrow, handle_repay, handle_supply, handle_withdraw};
use futures_util::future::try_join_all;
use shared::constants::XIPPLE_FI_CONTRACT_ADDRESS;
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

    let db = {
        let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
        let mut opt = ConnectOptions::new(db_url);
        opt.sqlx_logging(false);

        Database::connect(opt)
            .await
            .expect("can not connect to database")
    };

    let provider = {
        let rpc_url = Url::from_str("https://rpc-evm-sidechain.xrpl.org").expect("invalid rpc url");
        ProviderBuilder::new().on_http(rpc_url)
    };

    let key = Config::LatestScannedBlock;

    let mut latest_scanned_block = {
        let config = setting::find(&db, &key)
            .await
            .expect("failed to find latest_scanned_block")
            .expect("missing latest_scanned_block setting");

        u64::from_str(&config).expect("invalid latest_scanned_block, u64 expected")
    };

    let mut filter = Filter::new()
        .address(XIPPLE_FI_CONTRACT_ADDRESS)
        .from_block(latest_scanned_block)
        .to_block(latest_scanned_block + DISTANCE);

    loop {
        latest_scanned_block = scan(&provider, &db, &mut filter)
            .await
            .unwrap_or_else(|error| {
                tracing::error!("scan error {:#?}", error);
                latest_scanned_block
            });

        setting::set(&db, &key, latest_scanned_block.to_string())
            .await
            .expect("failed to set latest_scanned_block");

        tracing::info!("scanned to block {}", latest_scanned_block);

        tokio::time::sleep(Duration::from_secs(3)).await;
    }
}

async fn scan(
    provider: &RootProvider<Http<Client>>,
    db: &DatabaseConnection,
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

    let tasks = provider
        .get_logs(filter)
        .await?
        .into_iter()
        .map(|log| async move {
            let Ok(event) = XippleFiEvents::decode_log(&log.inner, true) else {
                return Ok(());
            };

            let hash =
                log.transaction_hash
                    .map(|hash| hash.to_string())
                    .ok_or(ScannerError::Custom(
                        "not found transaction hash from log".to_string(),
                    ))?;

            if event::find_by_hash(db, &hash).await?.is_some() {
                return Ok(());
            }

            let date = if let Some(block_timestamp) = log.block_timestamp {
                DateTime::from_timestamp(block_timestamp as i64, 0).ok_or(ScannerError::Custom(
                    "can not parse block timestamp".to_string(),
                ))?
            } else {
                let block_hash = log.block_hash.ok_or(ScannerError::Custom(
                    "not found block hash from log".to_string(),
                ))?;

                let block = provider
                    .get_block_by_hash(block_hash, BlockTransactionsKind::Full)
                    .await?
                    .ok_or(ScannerError::Custom(
                        "not found block from block hash".to_string(),
                    ))?;

                DateTime::from_timestamp(block.header.timestamp as i64, 0).ok_or(
                    ScannerError::Custom("can not parse block timestamp".to_string()),
                )?
            };

            match event.data {
                XippleFiEvents::Borrow(payload) => handle_borrow(db, hash, date, payload).await?,
                XippleFiEvents::Repay(payload) => handle_repay(db, hash, date, payload).await?,
                XippleFiEvents::Supply(payload) => handle_supply(db, hash, date, payload).await?,
                XippleFiEvents::Withdraw(payload) => {
                    handle_withdraw(db, hash, date, payload).await?
                }
            }

            Ok::<(), ScannerError>(())
        });

    try_join_all(tasks).await?;

    let next = to_block + 1;

    filter.block_option = FilterBlockOption::Range {
        from_block: BlockNumberOrTag::Number(next).into(),
        to_block: BlockNumberOrTag::Number(next + DISTANCE).into(),
    };

    Ok(to_block)
}
