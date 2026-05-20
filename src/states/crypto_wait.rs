pub struct CryptoWaitState {
    pub target_confirmations: u8,
}

impl CryptoWaitState {
    pub fn new(confirmations: u8) -> Self {
        Self { target_confirmations: confirmations }
    }

    pub fn display_status(&self, current: u8) {
        println!(
            "[UI Screen] Awaiting Settlement: [{}/{}] confirmations detected in mempool...", 
            current, 
            self.target_confirmations
        );
    }
}
