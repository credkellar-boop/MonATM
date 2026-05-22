#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod auth;
mod crypto;
mod hardware;
mod models;
mod states;
mod transactions;
mod controller;

use auth::card_reader::CardReader;
use hardware::cash_dispenser::CashDispenser;
use crypto::oracle::ExchangeRateOracle;
use crypto::wallet::HotWalletManager;
use states::{SystemContext, AtmState};
use transactions::crypto_offramp::{CryptoToCashOffRamp, OffRampTransaction};

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("=============================================");
    println!("        BOOTING MonATM PRODUCTION OS         ");
    println!("=============================================");

    let mut context = SystemContext {
        current_state: states::StateMachineStatus::Idle,
        session_id: None,
        verified_identity: None,
        selected_token: None,
        target_fiat_amount: 0,
    };
    
    let mut reader = CardReader::new();
    let mut dispenser = CashDispenser::new(50000);
    let oracle = ExchangeRateOracle;

    println!("\n[UI User Approaches Terminal...]");
    
    let card_token = reader.scan_card()?;
    println!("Card processing event received token: {:?}", card_token);
    
    println!("\n[UI Screen Prompt: Please enter your 4-digit PIN]");
    let current_state: Box<dyn AtmState> = Box::new(states::IdleState);
    
    let pin_input = "1234"; 
    let next_state = current_state.handle_input(&mut context, "card_inserted").await?;
    let verified_state = next_state.handle_input(&mut context, pin_input).await?;
    println!("Transitioned State Level to: {:?}", verified_state.status());

    context.selected_token = Some("USDC".to_string());
    let fiat_amount_requested = 200;
    context.target_fiat_amount = fiat_amount_requested;

    let deposit_address = HotWalletManager::generate_session_address("USDC");
    let spot_price = oracle.get_spot_price("USDC").await?;
    
    let token_amount_required = (fiat_amount_requested as f64) / (spot_price.to_string().parse::<f64>().unwrap_or(1.0));

    let qr_screen = states::crypto_qr::CryptoQrState::new(&deposit_address, token_amount_required);
    qr_screen.display_matrix_payload();

    let wait_state = states::crypto_wait::CryptoWaitState::new(1);
    wait_state.display_status(0);

    let dispenser_ref = &mut dispenser;
    let mut offramp = CryptoToCashOffRamp {
        fiat_requested: fiat_amount_requested as u32,
    };
    
    offramp.execute(dispenser_ref).await?;

    reader.eject_card();
    println!("\n=============================================");
    println!("       MonATM SESSION SECURELY CLOSED        ");
    println!("=============================================");

    Ok(())
}
