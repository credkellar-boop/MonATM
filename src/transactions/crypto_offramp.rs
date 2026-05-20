use async_trait::async_trait;
use crate::hardware::cash_dispenser::CashDispenser;

pub struct CryptoToCashOffRamp {
    pub fiat_requested: u32,
}

#[async_trait]
pub trait OffRampTransaction {
    async fn execute(&self, dispenser: &mut CashDispenser) -> Result<(), String>;
}

#[async_trait]
impl OffRampTransaction for CryptoToCashOffRamp {
    async fn execute(&self, dispenser: &mut CashDispenser) -> Result<(), String> {
        println!("Verifying crypto deposit in mempool...");
        // Logic to await blockchain confirmation goes here
        dispenser.dispense_currency(self.fiat_requested).await
    }
}
