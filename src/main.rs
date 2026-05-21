mod auth;
mod crypto;
mod hardware;
mod models;
mod states;
mod transactions;
mod controller;

use auth::card_reader::CardReader;
use auth::secure_auth::SecureAuthService;
use crypto::oracle::ExchangeRateOracle;
use crypto::wallet::HotWalletManager;
use hardware::cash_dispenser::CashDispenser;
use states::crypto_qr::CryptoQrState;
use states::crypto_wait::CryptoWaitState;
use states::tx_select::TxSelectState;
use transactions::crypto_offramp::{CryptoToCashOffRamp, OffRampTransaction};

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("==================================================");
    println!("          BOOTING MonATM PRODUCTION OS            ");
    println!("==================================================");

    // 1. Initialize Hardware Components
    let mut reader = CardReader::new();
    let mut dispenser = CashDispenser::new(50000); // Loaded with $50,000 cash
    let oracle = ExchangeRateOracle;

    // 2. Simulate User Interaction (Card Sweep & Pin Verification)
    println!("\n[User] Approaches terminal...");
    let card_token = reader.scan_card()?;
    
    println!("\n[User] Enters credentials on secure keypad...");
    let pin_is_valid = SecureAuthService::verify_pin(&card_token, "1234");
    
    if !pin_is_valid {
        reader.eject_card();
        return Err("ATM Security Alert: Invalid PIN entry.".to_string());
    }

    // 3. Render Dashboard State Menu
    TxSelectState::display_menu();
    println!("\n[User Selection] Option 4: Crypto Out-Ramp Requested ($200 Fiat Payout)");
    let fiat_amount_requested = 200;

    // 4. Generate Single-Use Session Escrow Wallet
    let deposit_address = HotWalletManager::generate_session_address("USDC");
    let target_rate = ExchangeRateOracle::get_spot_price("USDC").await?;
    let token_amount_string = (fiat_amount_requested as f32 / target_rate.to_string().parse::<f32>().unwrap()).to_string();

    // 5. Render Screen UI QR Invoice
    let qr_screen = CryptoQrState::new(&deposit_address, &token_amount_string);
    qr_screen.display_matrix_payload();

    // 6. Monitor Inbound Blockchain Mempool Blocks
    let wait_state = CryptoWaitState::new(1);
    wait_state.display_status(0);
    
    // Create the off-ramp instance and execute pipeline transaction mechanics
    let offramp_tx = CryptoToCashOffRamp { fiat_requested: fiat_amount_requested };
    offramp_tx.execute(&mut dispenser).await?;

    // 7. Session Cleanup
    wait_state.display_status(1);
    println!("\n[Transaction Complete] Please take your cash.");
    reader.eject_card();
    
    println!("==================================================");
    println!("          MonATM SESSION SECURELY CLOSED          ");
    println!("==================================================");
    Ok(())
}
mod hardware;
mod deposit;
mod crypto;
mod states;
// ... any other missing top-level modules