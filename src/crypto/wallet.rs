pub struct HotWalletManager;

impl HotWalletManager {
    pub fn generate_session_address(token_type: &str) -> String {
        let unique_id = uuid::Uuid::new_v4().to_string();
        format!("monatm_{}_{}", token_type.to_lowercase(), &unique_id[..8])
    }
}
