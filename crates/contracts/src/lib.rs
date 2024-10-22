pub mod defi;
pub mod error;
pub mod oracle;

use alloy::{
    network::{AnyNetwork, EthereumWallet},
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, RootProvider,
    },
    sol,
    transports::http::{Client, Http},
};

sol!(
    #[sol(rpc)]
    ORACLE,
    "src/abis/oracle.abi.json"
);

sol!(
    #[sol(rpc)]
    DEFI,
    "src/abis/defi.abi.json"
);

pub type OracaleProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider<Http<Client>, AnyNetwork>,
    Http<Client>,
    AnyNetwork,
>;
