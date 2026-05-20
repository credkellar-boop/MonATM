pub struct CashDispenser {
    pub available_fiat: u32,
}

impl CashDispenser {
    pub fn new(initial_load: u32) -> Self {
        Self { available_fiat: initial_load }
    }

    pub async fn dispense_currency(&mut self, amount: u32) -> Result<(), String> {
        if self.available_fiat >= amount {
            self.available_fiat -= amount;
            println!("Dispensing ${}", amount);
            Ok(())
        } else {
            Err("Insufficient physical funds".to_string())
        }
    }
}
