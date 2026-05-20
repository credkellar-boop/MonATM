pub struct CryptoQrState {
    pub current_invoice_uri: String,
}

impl CryptoQrState {
    pub fn new(address: &str, amount: &str) -> Self {
        Self {
            current_invoice_uri: format!("crypto:{}?amount={}", address, amount),
        }
    }

    pub fn display_matrix_payload(&self) {
        println!("[UI Screen] Target Wallet Out-Ramp Established.");
        println!("URI payload target: {}", self.current_invoice_uri);
    }
}