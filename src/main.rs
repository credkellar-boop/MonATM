mod auth;
mod crypto;
mod hardware;
mod models;
mod states;
mod transactions;
mod controller;

use controller::AtmController;

#[tokio::main]
async fn main() {
    println!("Booting MonATM Core System...");
    let mut terminal = AtmController::new();
    terminal.run().await;
}
