use rust_decimal::Decimal;

pub struct Account {
    pub account_id: String,
    pub fiat_balance: u32,
    pub linked_crypto_wallets: Vec<String>,
}
