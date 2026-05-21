// src/transactions/balance.rs

pub fn get_balance() -> f64 {
    // TODO: Implement actual balance retrieval logic for MonATM
    0.0
}

pub fn display_balance() {
    let current_balance = get_balance();
    println!("Current Balance: ${:.2}", current_balance);
}
