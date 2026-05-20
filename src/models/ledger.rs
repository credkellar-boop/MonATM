pub struct LocalLedgerEntry {
    pub entry_id: u64,
    pub timestamp: u64,
    pub transaction_type: String,
    pub delta: i32,
}
