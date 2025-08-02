use crate::error::{Result, StockCalcError, ValidationError};
use crate::models::{StockData, StockAnalysis, InvestmentScale, StockStatus};
use chrono::Utc;

pub struct StockCalculator;

impl StockCalculator {
    pub fn validate_input(
        code: &str,
        quantity: f64,
        avg_price: f64,
        target_profit: f64,
        max_loss: f64,
    ) -> Result<()> {
        // 股票代码验证
        if !code.chars().all(|c| c.is_alphanumeric()) {
            return Err(StockCalcError::ValidationError(
                ValidationError::InvalidStockCode.to_string()
            ));
        }

        // 数值验证
        if quantity <= 0.0 {
            return Err(StockCalcError::ValidationError(
                ValidationError::InvalidQuantity.to_string()
            ));
        }

        if avg_price <= 0.0 {
            return Err(StockCalcError::ValidationError(
                ValidationError::InvalidPrice.to_string()
            ));
        }

        if target_profit <= 0.0 {
            return Err(StockCalcError::ValidationError(
                ValidationError::InvalidTargetProfit.to_string()
            ));
        }

        if max_loss <= 0.0 {
            return Err(StockCalcError::ValidationError(
                ValidationError::InvalidMaxLoss.to_string()
            ));
        }

        Ok(())
    }

    pub fn calculate_analysis(
        stock_data: &StockData,
        current_price: f64,
    ) -> StockAnalysis {
        let investment_amount = stock_data.quantity * stock_data.avg_price;
        let target_price = stock_data.avg_price + (stock_data.target_profit / stock_data.quantity);
        let stop_loss_price = stock_data.avg_price - (stock_data.max_loss / stock_data.quantity);
        let current_profit = (current_price - stock_data.avg_price) * stock_data.quantity;
        let profit_ratio = ((current_price - stock_data.avg_price) / stock_data.avg_price) * 100.0;
        let distance_to_target = ((target_price - current_price) / current_price) * 100.0;
        let distance_to_stop_loss = ((current_price - stop_loss_price) / current_price) * 100.0;
        let risk_reward_ratio = stock_data.target_profit / stock_data.max_loss;

        let investment_scale = Self::classify_investment_scale(investment_amount);
        let status = Self::determine_stock_status(
            current_price,
            target_price,
            stop_loss_price,
            profit_ratio,
        );

        StockAnalysis {
            code: stock_data.code.clone(),
            current_price,
            quantity: stock_data.quantity,
            avg_price: stock_data.avg_price,
            target_profit: stock_data.target_profit,
            max_loss: stock_data.max_loss,
            target_price,
            stop_loss_price,
            current_profit,
            profit_ratio,
            distance_to_target,
            distance_to_stop_loss,
            risk_reward_ratio,
            investment_amount,
            investment_scale,
            status,
            last_updated: Utc::now(),
        }
    }

    fn classify_investment_scale(investment_amount: f64) -> InvestmentScale {
        match investment_amount {
            amount if amount < 10_000.0 => InvestmentScale::Small,
            amount if amount <= 100_000.0 => InvestmentScale::Medium,
            _ => InvestmentScale::Large,
        }
    }

    fn determine_stock_status(
        current_price: f64,
        target_price: f64,
        stop_loss_price: f64,
        profit_ratio: f64,
    ) -> StockStatus {
        if current_price >= target_price {
            StockStatus::AtTarget
        } else if current_price <= stop_loss_price {
            StockStatus::AtStopLoss
        } else if profit_ratio > 0.0 {
            if current_price >= target_price * 0.95 {
                StockStatus::NearTarget
            } else {
                StockStatus::Profitable
            }
        } else {
            if current_price <= stop_loss_price * 1.05 {
                StockStatus::NearStopLoss
            } else {
                StockStatus::Loss
            }
        }
    }

    pub fn format_currency(amount: f64) -> String {
        format!("¥{:.3}", amount)
    }

    pub fn format_percentage(percentage: f64) -> String {
        format!("{:.2}%", percentage)
    }

    pub fn get_risk_level(risk_reward_ratio: f64) -> &'static str {
        match risk_reward_ratio {
            ratio if ratio >= 3.0 => "优秀",
            ratio if ratio >= 2.0 => "良好",
            ratio if ratio >= 1.5 => "一般",
            _ => "较差",
        }
    }

    pub fn get_investment_scale_text(scale: &InvestmentScale) -> &'static str {
        match scale {
            InvestmentScale::Small => "小额",
            InvestmentScale::Medium => "中等",
            InvestmentScale::Large => "大额",
        }
    }

    pub fn get_status_emoji(status: &StockStatus) -> &'static str {
        match status {
            StockStatus::Profitable => "📈",
            StockStatus::Loss => "📉",
            StockStatus::AtTarget => "🎉",
            StockStatus::AtStopLoss => "⚠️",
            StockStatus::NearTarget => "🎯",
            StockStatus::NearStopLoss => "🚨",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::StockData;
    use chrono::Utc;

    #[test]
    fn test_validate_input() {
        // 有效输入
        assert!(StockCalculator::validate_input("000001", 1000.0, 15.5, 5000.0, 2000.0).is_ok());

        // 无效股票代码
        assert!(StockCalculator::validate_input("000-001", 1000.0, 15.5, 5000.0, 2000.0).is_err());

        // 无效数量
        assert!(StockCalculator::validate_input("000001", 0.0, 15.5, 5000.0, 2000.0).is_err());

        // 无效价格
        assert!(StockCalculator::validate_input("000001", 1000.0, 0.0, 5000.0, 2000.0).is_err());
    }

    #[test]
    fn test_calculate_analysis() {
        let stock_data = StockData {
            code: "000001".to_string(),
            quantity: 1000.0,
            avg_price: 15.5,
            target_profit: 5000.0,
            max_loss: 2000.0,
            last_updated: Utc::now(),
        };

        let analysis = StockCalculator::calculate_analysis(&stock_data, 16.2);

        assert_eq!(analysis.target_price, 20.5); // 15.5 + (5000/1000)
        assert_eq!(analysis.stop_loss_price, 13.5); // 15.5 - (2000/1000)
        assert!((analysis.current_profit - 700.0).abs() < 0.01); // (16.2 - 15.5) * 1000
        assert!((analysis.profit_ratio - 4.52).abs() < 0.01); // 约4.52%
        assert_eq!(analysis.investment_amount, 15500.0); // 1000 * 15.5
    }

    #[test]
    fn test_classify_investment_scale() {
        assert!(matches!(
            StockCalculator::classify_investment_scale(5000.0),
            InvestmentScale::Small
        ));
        assert!(matches!(
            StockCalculator::classify_investment_scale(50000.0),
            InvestmentScale::Medium
        ));
        assert!(matches!(
            StockCalculator::classify_investment_scale(150000.0),
            InvestmentScale::Large
        ));
    }
} 