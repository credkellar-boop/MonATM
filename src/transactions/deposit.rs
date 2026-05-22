// Inside src/transactions/deposit.rs
pub struct DepositEngine;

impl DepositEngine {
    pub fn process_deposit(amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be greater than zero.".to_string());
        }
        println!("Successfully processed deposit of ${:.2}", amount);
        Ok(())
    }
}
