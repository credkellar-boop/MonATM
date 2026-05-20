pub struct PinEntryState {
    pub attempts_remaining: u8,
}

impl PinEntryState {
    pub fn new() -> Self {
        Self { attempts_remaining: 3 }
    }

    pub fn process_attempt(&mut self, is_valid: bool) -> Result<(), String> {
        if is_valid {
            println!("[State: PIN Entry] Authentication successful. Transitioning...");
            Ok(())
        } else {
            self.attempts_remaining -= 1;
            if self.attempts_remaining == 0 {
                Err("Card locked due to excessive invalid attempts.".to_string())
            } else {
                Err(format!("Invalid PIN. Attempts left: {}", self.attempts_remaining))
            }
        }
    }
}
