use crate::hardware::cash_dispenser::CashDispenser;

pub struct StandardWithdrawal {
    pub amount_requested: u32,
}

impl StandardWithdrawal {
    pub async fn execute_fiat_payout(&self, dispenser: &mut CashDispenser) -> Result<(), String> {
        dispenser.dispense_currency(self.amount_requested).await
    }
}
