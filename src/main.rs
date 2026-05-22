// src/main.rs

mod auth;
mod crypto;
mod hardware;
mod models;
mod states;
mod transactions;
mod controller;

use hardware::card_reader::CardReader;
use hardware::cash_dispenser::CashDispenser;
use crypto::oracle::ExchangeRateOracle;
use crypto::wallet::HotWalletManager;
use crypto::client::CryptoClient;
use states::{SystemContext, AtmState};
use transactions::crypto_offramp::CryptoToCashOffRamp;

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("=============================================");
    println!("        BOOTING MonATM PRODUCTION OS         ");
    println!("=============================================");

    // 1. Initialize Context Systems
    let mut context = SystemContext {
        current_state: states::StateMachineStatus::Idle,
        session_id: None,
        verified_identity: None,
        selected_token: None,
        target_fiat_amount: 0,
    };
    
    let mut reader = CardReader::new();
    let mut dispenser = CashDispenser::new(50000); // Initialize with $50,000 cash reserves
    let oracle = ExchangeRateOracle;

    // 2. Simulate User Interaction Sequence driving the engine
    println!("\n[UI User Approaches Terminal...]");
    
    // Simulate Card Insertion triggering state pipeline
    let card_token = reader.scan_card();
    println!("Card processing event received: {:?}", card_token);
    
    // Move into PIN entry verification
    println!("\n[UI Screen Prompt: Please enter your 4-digit PIN]");
    let current_state: Box<dyn AtmState> = Box::new(states::IdleState);
    
    // Pass raw peripheral interface inputs through the concrete machine states
    let pin_input = "1234"; 
    let next_state = current_state.handle_input(&mut context, pin_input).await?;
    println!("Transitioned State Level to: {:?}", next_state.status());

    // 3. Execution Pipeline for single-use Monad Escrow Settlement (Option 4: Cryto Off-Ramp)
    context.selected_token = Some("USDC".to_string());
    let fiat_amount_requested = 200;
    context.target_fiat_amount = fiat_amount_requested;

    let deposit_address = HotWalletManager::generate_session_address("USDC");
    let spot_price = oracle.get_spot_price("USDC").await?;
    
    let token_amount_required = (fiat_amount_requested as f64) / (spot_price.to_string().parse::<f64>().unwrap());

    // Render Matrix Payload Destination QR Code
    let qr_screen = crypto::crypto_qr::CryptoQrState::new(&deposit_address, token_amount_required);
    qr_screen.display_matrix_payload();

    // Track Inbound Mempool verification blocks
    let wait_state = states::crypto_wait::CryptoWaitState::new(1);
    wait_state.display_status(0);

    // 4. Perform Atomic Cash Ledger Off-ramp Execution
    let mut offramp = CryptoToCashOffRamp {
        fiat_requested: fiat_amount_requested as u32,
    };
    
    use transactions::crypto_offramp::OffRampTransaction;
    offramp.execute(&mut dispenser).await?;

    // 5. Clean up Physical Hardware state
    reader.eject_card();
    println!("\n=============================================");
    println!("       MonATM SESSION SECURELY CLOSED        ");
    println!("=============================================");

    Ok(())
}
