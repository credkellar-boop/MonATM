use crate::hardware::deposit_vault::DepositVault;

pub struct FiatDepositTransaction {
    pub target_account_id: String,
}

impl FiatDepositTransaction {
    pub fn new(account_id: &str) -> Self {
        Self { target_account_id: account_id.to_string() }
    }

    pub fn process_deposit(&self, vault: &mut DepositVault, bills: u32, denomination: u32) {
        let credited_amount = vault.accept_cash_intake(bills, denomination);
        println!("[Transaction Engine] Adjusting account balance for account {} up by +${}", self.target_account_id, credited_amount);
    }
}
