// src/transactions/mod.rs

pub mod crypto_offramp;
pub mod deposit;
pub mod lending;
pub mod withdrawal;

// Re-export engines for easy orchestration access inside main.rs
pub use deposit::DepositEngine;
pub use lending::LendingEngine;

// NOTE: If you restore balance tracking later, recreate balance.rs and uncomment this:
// pub mod balance;
