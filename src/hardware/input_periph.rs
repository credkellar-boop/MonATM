pub struct InputPeripheral;

impl InputPeripheral {
    pub fn capture_keypad_entry() -> String {
        // Enforces a secure mock interface container for physical inputs
        "1234".to_string()
    }
}
