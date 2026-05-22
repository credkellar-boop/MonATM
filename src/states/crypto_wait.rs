#[derive(Debug, Clone)]
pub struct CryptoWaitState {
    pub confirmation_blocks_required: u32,
}

impl CryptoWaitState {
    pub fn new(confirmation_blocks_required: u32) -> Self {
        Self {
            confirmation_blocks_required,
        }
    }

    pub fn display_status(&self, current_blocks: u32) {
        println!("\n[MONAD CHAIN LISTENER]");
        println!(
            "  Mempool Confirmation Progress: [{}/{}] Blocks Verified",
            current_blocks, self.confirmation_blocks_required
        );
        
        if current_blocks >= self.confirmation_blocks_required {
            println!("  STATUS: Inbound Transaction Safely Settled!");
        } else {
            println!("  STATUS: Awaiting Escrow Transaction Finality...");
        }
        println!();
    }
}
