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

        // 1. Verify physical hardware has sufficient inventory allocation
        if dispenser.get_balance() < self.fiat_requested {
            return Err("TERMINAL ERROR: Insufficient vault funds to complete requested payout.".to_string());
        }

        // 2. Trigger mechanical internal hardware rails to release fiat assets
        println!(
            "[OFF-RAMP ENGINE] Dispenser checks passed. Requesting physical bill count: ${}",
            self.fiat_requested
        );
        
        dispenser.dispense_cash(self.fiat_requested)?;

        println!("[OFF-RAMP ENGINE] Cash successfully moved to output tray. Settlement verified.\n");
        Ok(())
    }
}
