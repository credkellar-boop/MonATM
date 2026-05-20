pub struct TxSelectState;

impl TxSelectState {
    pub fn display_menu() {
        println!("\n--- MonATM MAIN MENU ---");
        println!("1. Cash Withdrawal");
        println!("2. Fiat Deposit");
        println!("3. Balance Inquiry");
        println!("4. Crypto Out-Ramp (Cash-out Crypto)");
        println!("5. Cancel & Eject Card");
        println!("------------------------");
    }
}
