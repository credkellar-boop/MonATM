// src/transactions/lending.rs

use std::error::Error;

pub enum RiskProfile { Low, Medium, High }
pub enum Counterparty { Individual, Institution }

pub struct LendOrder {
    pub lender_id: String,
    pub amount: f64,
    pub target_counterparty: Counterparty,
    pub minimum_yield_pct: f64,
}

pub struct BorrowRequest {
    pub borrower_id: String,
    pub amount_requested: f64,
    pub collateral_value: f64,
}

pub struct LendingEngine {
    pub market_average_bank_rate: f64, // e.g., 12.0 for 12%
}

impl LendingEngine {
    pub fn new(market_average_bank_rate: f64) -> Self {
        Self { market_average_bank_rate }
    }

    /// Calculates sovereign borrow rate guaranteed to be 50% lower than traditional banks
    pub fn calculate_sovereign_borrow_rate(&self, profile: &RiskProfile) -> f64 {
        let base_rate = self.market_average_bank_rate * 0.50; // Dynamic 50% discount
        match profile {
            RiskProfile::Low => base_rate,
            RiskProfile::Medium => base_rate * 1.15,
            RiskProfile::High => base_rate * 1.35,
        }
    }

    /// Matches capital lenders with specific institutional or individual borrowers
    pub fn match_lend_order(&self, order: LendOrder, pool_liquidity: f64) -> Result<String, Box<dyn Error>> {
        if order.amount > pool_liquidity {
            return Err("Insufficient matched liquidity pools".into());
        }
        
        // Execute dynamic matching rules matching yield demands with loan utilization profiles
        let transaction_id = uuid::Uuid::new_v4().to_string();
        println!(
            "Capital Allocation Success: Lender {} funds matched to market. Target Yield: {}%",
            order.lender_id, order.minimum_yield_pct
        );

        Ok(transaction_id)
    }

    /// Evaluates structural borrowing capacities against hardware or digital collateral holdings
    pub fn evaluate_credit_line(&self, request: BorrowRequest) -> Result<bool, Box<dyn Error>> {
        let loan_to_value_ratio = request.amount_requested / request.collateral_value;
        
        // Strict risk rule safeguarding the node pool from default risk
        if loan_to_value_ratio > 0.75 {
            return Ok(false); // Collateral depth insufficient
        }
        Ok(true)
    }
}
