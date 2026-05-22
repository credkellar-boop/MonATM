#[derive(Debug, Clone)]
pub struct CardReader {
    pub card_inserted: bool,
}

impl CardReader {
    pub fn new() -> Self {
        Self { card_inserted: false }
    }

    pub fn scan_card(&mut self) -> Result<String, String> {
        self.card_inserted = true;
        println!("[Hardware] Card successfully read via physical bezel sensor.");
        Ok("MOCKED_CARD_TOKEN_99812A".to_string())
    }

    pub fn eject_card(&mut self) {
        self.card_inserted = false;
        println!("[Hardware] Card physically ejected from slot.");
    }
}
