use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockData {
    pub code: String,
    pub quantity: f64,
    pub avg_price: f64,
    pub target_profit: f64,
    pub max_loss: f64,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockAnalysis {
    pub code: String,
    pub current_price: f64,
    pub quantity: f64,
    pub avg_price: f64,
    pub target_profit: f64,
    pub max_loss: f64,
    pub target_price: f64,
    pub stop_loss_price: f64,
    pub current_profit: f64,
    pub profit_ratio: f64,
    pub distance_to_target: f64,
    pub distance_to_stop_loss: f64,
    pub risk_reward_ratio: f64,
    pub investment_amount: f64,
    pub investment_scale: InvestmentScale,
    pub status: StockStatus,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvestmentScale {
    Small,      // < 1万
    Medium,     // 1-10万
    Large,      // > 10万
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StockStatus {
    Profitable,     // 盈利
    Loss,           // 亏损
    AtTarget,       // 达到目标
    AtStopLoss,     // 达到止损
    NearTarget,     // 接近目标
    NearStopLoss,   // 接近止损
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockDatabase {
    pub stocks: std::collections::HashMap<String, StockData>,
}

impl StockDatabase {
    pub fn new() -> Self {
        Self {
            stocks: std::collections::HashMap::new(),
        }
    }

    pub fn add_stock(&mut self, stock: StockData) {
        self.stocks.insert(stock.code.clone(), stock);
    }

    pub fn remove_stock(&mut self, code: &str) -> Option<StockData> {
        self.stocks.remove(code)
    }

    pub fn get_stock(&self, code: &str) -> Option<&StockData> {
        self.stocks.get(code)
    }

    pub fn list_stocks(&self) -> Vec<&StockData> {
        self.stocks.values().collect()
    }
}

impl Default for StockDatabase {
    fn default() -> Self {
        Self::new()
    }
} 