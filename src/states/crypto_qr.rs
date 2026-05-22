#[derive(Debug, Clone)]
pub struct CryptoQrState {
    pub deposit_address: String,
    pub amount_required: f64,
}

impl CryptoQrState {
    pub fn new(deposit_address: &str, amount_required: f64) -> Self {
        Self {
            deposit_address: deposit_address.to_string(),
            amount_required,
        }
    }

    pub fn display_matrix_payload(&self) {
        println!("\n---------------------------------------------");
        println!("       GENERATING ON-CHAIN ESCROW QR         ");
        println!("---------------------------------------------");
        println!("  Send Exactly: {:.6} Tokens", self.amount_required);
        println!("  To Destination: {}", self.deposit_address);
        println!("  [ MATRIX QR CODE PAYLOAD STUB ]");
        println!("---------------------------------------------\n");
    }
}
