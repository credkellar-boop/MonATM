use async_trait::async_trait;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum StateMachineStatus {
    Idle,
    PinEntry,
    TxSelect,
    Dispensing,
    Complete,
    Failed(String),
}

pub struct SystemContext {
    pub current_status: StateMachineStatus,
    pub session_id: Option<String>,
    pub verified_identity: Option<String>,
    pub selected_token: Option<String>,
    pub target_fiat_amount: u64,
}

#[async_trait]
pub trait AtmState: Debug {
    async fn handle_input(&self, context: &mut SystemContext, input: &str) -> Result<Box<dyn AtmState>, String>;
    fn status(&self) -> StateMachineStatus;
}

// --- IDLE STATE ---
#[derive(Debug)]
pub struct IdleState;

#[async_trait]
impl AtmState for IdleState {
    async fn handle_input(&self, context: &mut SystemContext, input: &str) -> Result<Box<dyn AtmState>, String> {
        if input == "card_inserted" {
            context.session_id = Some(uuid::Uuid::new_v4().to_string());
            context.current_status = StateMachineStatus::PinEntry;
            Ok(Box::new(PinEntryState))
        } else {
            Ok(Box::new(IdleState))
        }
    }
    fn status(&self) -> StateMachineStatus { StateMachineStatus::Idle }
}

// --- PIN ENTRY STATE ---
#[derive(Debug)]
pub struct PinEntryState;

#[async_trait]
impl AtmState for PinEntryState {
    async fn handle_input(&self, context: &mut SystemContext, input: &str) -> Result<Box<dyn AtmState>, String> {
        // Simple 4-digit mock validation for identity isolation
        if input.len() == 4 && input.chars().all(|c| c.is_numeric()) {
            context.verified_identity = Some(format!("user_pubkey_hash_{}", input));
            context.current_status = StateMachineStatus::TxSelect;
            Ok(Box::new(TxSelectState))
        } else {
            return Err("Invalid PIN configuration. Try again.".to_string());
        }
    }
    fn status(&self) -> StateMachineStatus { StateMachineStatus::PinEntry }
}

// --- TRANSACTION SELECT STATE ---
#[derive(Debug)]
pub struct TxSelectState;

#[async_trait]
impl AtmState for TxSelectState {
    async fn handle_input(&self, context: &mut SystemContext, input: &str) -> Result<Box<dyn AtmState>, String> {
        // Expected format: "TOKEN:AMOUNT" (e.g., "USDT:100")
        let parts: Vec<&str> = input.split(':').collect();
        if parts.len() == 2 {
            context.selected_token = Some(parts[0].to_string());
            context.target_fiat_amount = parts[1].parse::<u64>().map_err(|_| "Invalid amount requested")?;
            context.current_status = StateMachineStatus::Complete;
            Ok(Box::new(IdleState)) // Cycle routes back to ready state post execution
        } else {
            Err("Format selection missing properties. Use 'TOKEN:AMOUNT'.".to_string())
        }
    }
    fn status(&self) -> StateMachineStatus { StateMachineStatus::TxSelect }
}