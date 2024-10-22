use alloy::sol;

sol! {
    #[derive(Debug)]
    enum InterestRateMode {NONE, STABLE, VARIABLE}

    #[sol(rpc)]
    #[derive(Debug)]
    event Borrow(
        address indexed reserve,
        address user,
        address indexed onBehalfOf,
        uint256 amount,
        InterestRateMode interestRateMode,
        uint256 borrowRate,
        uint16 indexed referralCode
    );
}
