use alloy::primitives::Address;

pub fn user_sign_message_key(address: &Address) -> String {
    format!("user_sign_message_{}", address)
}
