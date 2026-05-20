// src/transactions/mod.rs

pub mod balance;
pub mod crypto_offramp;
pub mod deposit;
pub mod withdrawal;
pub mod lending; // Registers the new lending/borrowing engine

// Re-export for easier engine orchestration access inside main.rs
pub use deposit::DepositEngine;
pub use lending::LendingEngine;
