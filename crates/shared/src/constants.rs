use std::str::FromStr;

use alloy::{
    primitives::{address, Address},
    signers::{
        k256::ecdsa::SigningKey,
        local::{LocalSigner, PrivateKeySigner},
    },
};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, ToSchema)]
pub enum Asset {
    #[serde(rename = "bitcoin")]
    BTC,
    #[serde(rename = "ethereum")]
    ETH,
    #[serde(rename = "xrp")]
    XRP,
    #[serde(rename = "binance-coin")]
    BNB,
    #[serde(rename = "tron")]
    TRON,
    #[serde(rename = "tether")]
    USDT,
}

pub struct AssetConfig {
    pub id: Asset,
    pub coincap_id: &'static str,
    pub symbol: &'static str,
    pub heartbeat: u32,
    pub threshhold: f32,
    pub apy: f32,
    pub oracle_contract: Address,
    pub address: Address,
}

pub const ASSETS: [AssetConfig; 6] = [
    AssetConfig {
        id: Asset::BTC,
        coincap_id: "bitcoin",
        symbol: "BTC",
        threshhold: 0.5,
        apy: 0.01,
        heartbeat: 3600, //1h
        address: address!("06d7354F56209fa461B27A173316013EcC4a4c99"),
        oracle_contract: address!("8B84bdC3Ac0db29003625dc2FdF9902b80e2484F"),
    },
    AssetConfig {
        id: Asset::ETH,
        coincap_id: "ethereum",
        symbol: "ETH",
        threshhold: 0.5,
        apy: 0.01,
        heartbeat: 3600, //1h
        address: address!("78AE63017E18520cf63CbA0a5CF190d7f04Cb3f6"),
        oracle_contract: address!("C6a1f4925676E0f81f871C53d2C5A7Cff7B773c6"),
    },
    AssetConfig {
        id: Asset::XRP,
        coincap_id: "xrp",
        symbol: "XRP",
        heartbeat: 86400, //24h
        threshhold: 0.5,
        apy: 0.5,
        address: address!("21fa8610CBD3a1a45bCB1DbE933052EBF9e3dd52"),
        oracle_contract: address!("50E67748dBdb608bE5b85d97b0Da72313f7Faf4f"),
    },
    AssetConfig {
        id: Asset::TRON,
        coincap_id: "tron",
        symbol: "TRX",
        threshhold: 0.2,
        apy: 0.5,
        heartbeat: 86400, //24h
        address: address!("880e0C475DeD4214De466891c5FBD61747b67083"),
        oracle_contract: address!("7D2e4B489a9058E728Bd9B63b23251A29f0Ed246"),
    },
    AssetConfig {
        id: Asset::BNB,
        coincap_id: "binance-coin",
        symbol: "BNB",
        threshhold: 0.5,
        apy: 0.6,
        heartbeat: 86400, //24h
        address: address!("e2C179BB9e31Cd6f16142D1C8d2dDB7458b371Ca"),
        oracle_contract: address!("02f6C887a1C0857bF7106c02FAeF05d46Ba6aBEf"),
    },
    AssetConfig {
        id: Asset::USDT,
        coincap_id: "tether",
        symbol: "USDT",
        threshhold: 0.25,
        apy: 1.0,
        heartbeat: 86400, //24h
        address: address!("cD84fcd2964612D1585F1494B8Ed4F1Ae29D32AC"),
        oracle_contract: address!("4f3110350D0F6510F3bA7792d9E1be68D5937c9A"),
    },
];

impl Asset {
    pub fn from_coincap_id(id: &str) -> Self {
        match id {
            "bitcoin" => Asset::BTC,
            "ethereum" => Asset::ETH,
            "xrp" => Asset::XRP,
            "binance-coin" => Asset::BNB,
            "tether" => Asset::USDT,
            "tron" => Asset::TRON,
            other => panic!("unsupported asset {}", other),
        }
    }

    pub fn as_config(&self) -> &'static AssetConfig {
        ASSETS
            .iter()
            .find(|asset| &asset.id == self)
            .expect("impossible")
    }

    pub fn owner(&self) -> LocalSigner<SigningKey> {
        let key = match self {
            Asset::BTC => "BTC_ORACLE_OWNER_PK",
            Asset::ETH => "ETH_ORACLE_OWNER_PK",
            Asset::XRP => "XRP_ORACLE_OWNER_PK",
            Asset::TRON => "TRON_ORACLE_OWNER_PK",
            Asset::BNB => "BNB_ORACLE_OWNER_PK",
            Asset::USDT => "USDT_ORACLE_OWNER_PK",
        };

        let owner_pk = std::env::var(key).unwrap_or_else(|_| panic!("missing {} env", key));

        PrivateKeySigner::from_str(&owner_pk).expect("invalid private key")
    }
}
