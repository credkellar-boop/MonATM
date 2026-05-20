pub struct AtmController {
    pub is_running: bool,
}

impl AtmController {
    pub fn new() -> Self {
        Self { is_running: true }
    }

    pub async fn run(&mut self) {
        println!("MonATM State Machine Initialized. Awaiting input...");
        while self.is_running {
            // Main state machine event loop goes here
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}
