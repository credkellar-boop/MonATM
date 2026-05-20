pub struct IdleState;

impl IdleState {
    pub fn handle_event(&self) {
        println!("[State: Idle] Terminal active. Loop checking card slot insertion or touchscreen input prompts...");
    }
}
