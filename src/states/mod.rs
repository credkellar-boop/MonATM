// src/states/mod.rs

use async_trait::async_trait;
use std::fmt::Debug;
use uuid::Uuid;

use crate::auth::secure_auth::SecureAuthService;

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
    pub current_state: StateMachineStatus,
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

// ==========================================
// 1. IDLE STATE
// ==========================================
#[derive(Debug)]
pub struct IdleState;

#[async_trait]
impl AtmState for IdleState {
    async fn handle_input(&self, context: &mut SystemContext, input: &str) -> Result<Box<dyn AtmState>, String> {
        if input == "card_inserted" {
            context.session_id = Some(Uuid::new_v4().to_string());
            context.current_state = StateMachineStatus::PinEntry;
            Ok(Box::new(PinEntryState))
        } else {
            Ok(Box::new(IdleState))
        }
    }

    fn status(&self) -> StateMachineStatus {
        StateMachineStatus::Idle
    }
}

// ==========================================
// 2. PIN ENTRY STATE
// ==========================================
#[derive(Debug)]
pub struct PinEntryState;

#[async_trait]
impl AtmState for PinEntryState {
    async fn handle_input(&self, context: &mut SystemContext, input: &str) -> Result<Box<dyn AtmState>, String> {
        // Enforce basic structural format matching
        if input.len() == 4 && input.chars().all(|c| c.is_numeric()) {
            
            // Extract token cached from card reader insertion (Fallback to default mock identifier if empty)
            let hardware_card_token = context.session_id.as_deref().unwrap_or("MOCKED_CARD_TOKEN_99812A");
            
            // Authenticate input securely against cryptographic verification sub-layer
            if SecureAuthService::verify_pin(hardware_card_token, input) {
                context.verified_identity = Some(format!("user_pubkey_hash_{}", input));
                context.current_state = StateMachineStatus::TxSelect;
                Ok(Box::new(TxSelectState))
            } else {
                return Err("Invalid PIN configuration. Try again.".to_string());
            }
        } else {
            return Err("Invalid PIN configuration. Try again.".to_string());
        }
    }

    fn status(&self) -> StateMachineStatus {
        StateMachineStatus::PinEntry
    }
}

// ==========================================
// 3. TRANSACTION SELECT STATE
// ==========================================
#[derive(Debug)]
pub struct TxSelectState;

#[async_trait]
impl AtmState for TxSelectState {
    async fn handle_input(&self, context: &mut SystemContext, input: &str) -> Result<Box<dyn AtmState>, String> {
        // Expected format: "TOKEN:AMOUNT" (e.g., "USDT:100" or "MON:250")
        let parts: Vec<&str> = input.split(':').collect();
        
        if parts.len() == 2 {
            let token = parts[0].to_string();
            let amount_res = parts[1].parse::<u64>();
            
            match amount_res {
                Ok(amount) => {
                    context.selected_token = Some(token);
                    context.target_fiat_amount = amount;
                    
                    // Transition machine status directly over into processing hardware delivery loop
                    context.current_state = StateMachineStatus::Dispensing;
                    Ok(Box::new(DispensingState))
                }
                Err(_) => {
                    context.current_state = StateMachineStatus::Idle;
                    Ok(Box::new(IdleState))
                }
            }
        } else {
            return Err("Format selection missing properties. Use 'TOKEN:AMOUNT'.".to_string());
        }
    }

    fn status(&self) -> StateMachineStatus {
        StateMachineStatus::TxSelect
    }
}

// ==========================================
// 4. DISPENSING STATE
// ==========================================
#[derive(Debug)]
pub struct DispensingState;

#[async_trait]
impl AtmState for DispensingState {
    async fn handle_input(&self, context: &mut SystemContext, _input: &str) -> Result<Box<dyn AtmState>, String> {
        // Cycle states back cleanly to ready status post successful mechanical execution
        context.current_state = StateMachineStatus::Complete;
        Ok(Box::new(CompleteState))
    }

    fn status(&self) -> StateMachineStatus {
        StateMachineStatus::Dispensing
    }
}

// ==========================================
// 5. TERMINAL COMPLETE STATE
// ==========================================
#[derive(Debug)]
pub struct CompleteState;

#[async_trait]
impl AtmState for CompleteState {
    async fn handle_input(&self, context: &mut SystemContext, _input: &str) -> Result<Box<dyn AtmState>, String> {
        // Automatically reset and wipe runtime parameters for the next customer iteration session
        context.session_id = None;
        context.verified_identity = None;
        context.selected_token = None;
        context.target_fiat_amount = 0;
        context.current_state = StateMachineStatus::Idle;
        Ok(Box::new(IdleState))
    }

    fn status(&self) -> StateMachineStatus {
        StateMachineStatus::Complete
    }
}
