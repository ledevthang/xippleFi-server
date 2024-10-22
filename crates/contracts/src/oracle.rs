use std::{str::FromStr, sync::LazyLock};

use alloy::{
    dyn_abi::DynSolValue,
    hex::FromHex,
    network::{AnyNetwork, EthereumWallet},
    primitives::{map::HashMap, Bytes, FixedBytes, U256},
    providers::ProviderBuilder,
    sol_types::SolEventInterface,
    transports::http::{reqwest::Url, Client, Http},
};
use shared::constants::{Asset, ASSETS};

use crate::{error::ContractError, OracaleProvider, ORACLE};

pub type OracleContract = ORACLE::ORACLEInstance<Http<Client>, OracaleProvider, AnyNetwork>;

pub static ORACLE_CONTRACTS: LazyLock<HashMap<Asset, OracleContract>> = LazyLock::new(|| {
    let rpc_url = Url::from_str("https://rpc-evm-sidechain.xrpl.org").expect("invalid rpc url");

    let mut map = HashMap::new();

    ASSETS.into_iter().for_each(|asset| {
        let wallet = EthereumWallet::from(asset.id.owner());

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .network::<AnyNetwork>()
            .on_http(rpc_url.clone());

        map.insert(asset.id, ORACLE::new(asset.oracle_contract, provider));
    });

    map
});

impl OracleContract {
    pub async fn write_asset(&self, price: U256) -> Result<String, ContractError> {
        let receipt = self.requestNewRound().send().await?.get_receipt().await?;

        let log = receipt
            .inner
            .inner
            .logs()
            .first()
            .map(|log| &log.inner)
            .ok_or(ContractError::Custom(
                "missing log from requestNewRound method".to_string(),
            ))?;

        let event = ORACLE::ORACLEEvents::decode_log(log, true)?;

        let ORACLE::ORACLEEvents::RoundRequested(event_data) = event.data else {
            return Err(ContractError::Custom("unexpected event".to_string()));
        };

        let next_round = event_data.round as u64 + 1;
        let now_in_seconds = chrono::Utc::now().timestamp() as u64;

        let latest_epoch = epoch_and_round_to_hex(now_in_seconds, next_round);

        let param_value = DynSolValue::Tuple(vec![
            DynSolValue::Uint(U256::from(now_in_seconds), 32),
            DynSolValue::Uint(price, 256),
        ])
        .abi_encode();

        let epoch_and_round = FixedBytes::from_hex(latest_epoch.as_bytes()).unwrap();

        let report = Bytes::from(param_value);

        let hash = self
            .transmit(epoch_and_round, report)
            .send()
            .await?
            .watch()
            .await?;

        Ok(hash.to_string())
    }
}

fn epoch_and_round_to_hex(epoch: u64, round: u64) -> String {
    let uint40_value = epoch << 8 | round;

    let hex_string = format!("{:x}", uint40_value);

    let padded_hex_string = format!("{:0>64}", hex_string);

    padded_hex_string
}
