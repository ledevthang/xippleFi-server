mod error;
mod heartbeat;
mod threshold;

use database::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use error::OracleError;
use futures_util::StreamExt;
use heartbeat::*;
use serde::Deserialize;
use shared::constants::{Asset, ASSETS};
use threshold::*;
use tokio_tungstenite::tungstenite::Message;
use tracing_subscriber::EnvFilter;

pub type PriceStr = String;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("failed to load env");

    let filter = EnvFilter::builder()
        .from_env()
        .unwrap()
        .add_directive("oracle=debug".parse().unwrap());

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let db = {
        let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
        let mut opt = ConnectOptions::new(db_url);
        opt.sqlx_logging(false);

        Database::connect(opt)
            .await
            .expect("can not connect to database")
    };

    loop {
        tracing::info!("🦀 oracle is running");

        socket(&db)
            .await
            .unwrap_or_else(|e| tracing::error!("socket error >> {:#?}", e));
    }
}

async fn socket(db: &DatabaseConnection) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    let assets = ASSETS
        .iter()
        .map(|asset| asset.coincap_id)
        .collect::<Vec<&str>>()
        .join(",");

    let (stream, _) =
        tokio_tungstenite::connect_async(format!("wss://ws.coincap.io/prices?assets={}", assets))
            .await?;

    let (_, mut reader) = stream.split();

    while let Some(message) = reader.next().await {
        if let Message::Text(message) = message? {
            handle_message(message, db)
                .await
                .unwrap_or_else(|e| tracing::error!("crawl error {:#?}", e));
        }
    }

    Ok(())
}

async fn handle_message(message: String, db: &DatabaseConnection) -> Result<(), OracleError> {
    let message = serde_json::from_str::<CoinCapMessage>(&message)
        .map_err(|e| OracleError::DeserializeError(e.to_string()))?;

    let assets = [
        (Asset::BTC, message.bitcoin),
        (Asset::BNB, message.bnb),
        (Asset::ETH, message.ethereum),
        (Asset::TRON, message.tron),
        (Asset::USDT, message.usdt),
        (Asset::XRP, message.xrp),
    ]
    .into_iter()
    .filter_map(|(id, price)| price.map(|price| (id, price)));

    threshold(db, assets).await?;
    heartbeat(db).await?;

    Ok(())
}

#[derive(Deserialize)]
struct CoinCapMessage {
    bitcoin: Option<PriceStr>,
    xrp: Option<PriceStr>,
    ethereum: Option<PriceStr>,
    tron: Option<PriceStr>,
    #[serde(rename(deserialize = "binance-coin"))]
    bnb: Option<PriceStr>,
    #[serde(rename(deserialize = "tether"))]
    usdt: Option<PriceStr>,
}
