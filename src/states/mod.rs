// Append this or ensure it is at the top of src/states/mod.rs
pub mod crypto_qr;
pub mod crypto_wait;
pub mod idle;
pub mod pin_entry;
pub mod tx_select;

// Re-export the base definitions we merged earlier
pub use idle::IdleState;
pub use pin_entry::PinEntryState;
pub use tx_select::TxSelectState;
