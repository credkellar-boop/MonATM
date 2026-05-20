pub struct DepositVault {
    pub current_stored_checks: u32,
    pub internal_cash_total: u32,
}

impl DepositVault {
    pub fn new() -> Self {
        Self { current_stored_checks: 0, internal_cash_total: 10000 }
    }

    pub fn accept_cash_intake(&mut self, bill_count: u32, face_value: u32) -> u32 {
        let total_inserted = bill_count * face_value;
        self.internal_cash_total += total_inserted;
        println!("[Hardware Vault] Validated and stacked ${} in physical currency bills.", total_inserted);
        total_inserted
    }
}
