use alloy::sol;
use serde::Serialize;

sol! {
    #[sol(rpc)]
    #[derive(Debug)]
    #[derive(Serialize)]
    contract VirtualXippleFi {
        enum InterestRateMode {NONE, STABLE, VARIABLE}

        event Borrow(
            address indexed reserve,
            address user,
            address indexed onBehalfOf,
            uint256 amount,
            InterestRateMode interestRateMode,
            uint256 borrowRate,
            uint16 indexed referralCode
        );

        event Repay(
            address indexed reserve,
            address indexed user,
            address indexed repayer,
            uint256 amount,
            bool useATokens
        );

        event Withdraw(address indexed reserve, address indexed user, address indexed to, uint256 amount);

        event Supply(
            address indexed reserve,
            address user,
            address indexed onBehalfOf,
            uint256 amount
        );
    }
}
