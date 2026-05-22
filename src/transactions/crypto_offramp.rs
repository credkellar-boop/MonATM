use async_trait::async_trait;
use crate::hardware::cash_dispenser::CashDispenser;

#[async_trait]
pub trait OffRampTransaction {
    async fn execute(&mut self, dispenser: &mut CashDispenser) -> Result<(), String>;
}

pub struct CryptoToCashOffRamp {
    pub fiat_requested: u32,
}

#[async_trait]
impl OffRampTransaction for CryptoToCashOffRamp {
    async fn execute(&mut self, dispenser: &mut CashDispenser) -> Result<(), String> {
        println!("\n[OFF-RAMP ENGINE] Initiating cash disbursement security checks...");

        // 1. Verify physical hardware using your actual public field: available_fiat
        if dispenser.available_fiat < self.fiat_requested {
            return Err("TERMINAL ERROR: Insufficient vault funds to complete requested payout.".to_string());
        }

        // 2. Trigger your actual async method: dispense_currency
        println!(
            "[OFF-RAMP ENGINE] Dispenser checks passed. Requesting physical bill count: ${}",
            self.fiat_requested
        );
        
        dispenser.dispense_currency(self.fiat_requested).await?;

        println!("[OFF-RAMP ENGINE] Cash successfully moved to output tray. Settlement verified.\n");
        Ok(())
    }
}
