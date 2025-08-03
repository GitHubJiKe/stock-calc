use crate::error::{Result, StockCalcError};
use crate::config::AppConfig;
use crate::models::StockInfo;
use regex::Regex;
use std::time::Duration;

pub struct StockApi {
    client: reqwest::Client,
    config: AppConfig,
}

impl StockApi {
    pub fn new(config: AppConfig) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.api.timeout))
            .user_agent(&config.api.user_agent)
            .build()?;

        Ok(Self { client, config })
    }

    pub async fn fetch_stock_price(&self, stock_code: &str) -> Result<f64> {
        let url = format!("http://sqt.gtimg.cn/utf8/q={}", stock_code);
        
        for attempt in 1..=self.config.api.retry_count {
            match self.fetch_with_retry(&url).await {
                Ok(price) => return Ok(price),
                Err(e) => {
                    if attempt == self.config.api.retry_count {
                        return Err(e);
                    }
                    log::warn!("第{}次尝试失败，正在重试: {}", attempt, e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }

        Err(StockCalcError::ParseError("所有重试都失败了".to_string()))
    }

    async fn fetch_with_retry(&self, url: &str) -> Result<f64> {
        let response = self.client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(StockCalcError::ParseError(
                format!("HTTP错误: {}", response.status())
            ));
        }

        let text = response.text().await?;
        self.parse_stock_price(&text)
    }

    fn parse_stock_price(&self, response_text: &str) -> Result<f64> {
        // 使用正则表达式解析响应
        let re = Regex::new(r#"v_[^=]+="([^"]+)""#).map_err(|e| {
            StockCalcError::ParseError(format!("正则表达式错误: {}", e))
        })?;

        if let Some(captures) = re.captures(response_text) {
            if let Some(data) = captures.get(1) {
                let fields: Vec<&str> = data.as_str().split('~').collect();
                
                if fields.len() >= 4 {
                    if let Ok(price) = fields[3].parse::<f64>() {
                        if price > 0.0 {
                            return Ok(price);
                        }
                    }
                }
            }
        }

        Err(StockCalcError::ApiResponseError(
            format!("无法解析股票价格，响应: {}", response_text)
        ))
    }

    pub async fn validate_stock_code(&self, stock_code: &str) -> Result<bool> {
        // 基本格式验证
        if !stock_code.chars().all(|c| c.is_alphanumeric()) {
            return Ok(false);
        }

        // 尝试获取价格来验证股票代码
        match self.fetch_stock_price(stock_code).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub async fn fetch_stock_info(&self, stock_code: &str) -> Result<StockInfo> {
        let url = format!("http://sqt.gtimg.cn/utf8/q={}", stock_code);
        
        for attempt in 1..=self.config.api.retry_count {
            match self.fetch_stock_info_with_retry(&url).await {
                Ok(info) => return Ok(info),
                Err(e) => {
                    if attempt == self.config.api.retry_count {
                        return Err(e);
                    }
                    log::warn!("第{}次尝试失败，正在重试: {}", attempt, e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }

        Err(StockCalcError::ParseError("所有重试都失败了".to_string()))
    }

    async fn fetch_stock_info_with_retry(&self, url: &str) -> Result<StockInfo> {
        let response = self.client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(StockCalcError::ParseError(
                format!("HTTP错误: {}", response.status())
            ));
        }

        let text = response.text().await?;
        self.parse_stock_info(&text)
    }

    fn parse_stock_info(&self, response_text: &str) -> Result<StockInfo> {
        // 使用正则表达式解析响应
        let re = Regex::new(r#"v_[^=]+="([^"]+)""#).map_err(|e| {
            StockCalcError::ParseError(format!("正则表达式错误: {}", e))
        })?;

        if let Some(captures) = re.captures(response_text) {
            if let Some(data) = captures.get(1) {
                let fields: Vec<&str> = data.as_str().split('~').collect();
                
                if fields.len() >= 32 {
                    let name = fields[1].to_string();
                    let code = fields[2].to_string();
                    let current_price = fields[3].parse::<f64>().unwrap_or(0.0);
                    let yesterday_close = fields[4].parse::<f64>().unwrap_or(0.0);
                    let open_price = fields[5].parse::<f64>().unwrap_or(0.0);
                    let volume = fields[6].parse::<u64>().unwrap_or(0);
                    let turnover = fields[37].parse::<f64>().unwrap_or(0.0);
                    let high_price = fields[33].parse::<f64>().unwrap_or(0.0);
                    let low_price = fields[34].parse::<f64>().unwrap_or(0.0);
                    let change_amount = fields[31].parse::<f64>().unwrap_or(0.0);
                    let change_percent = fields[32].parse::<f64>().unwrap_or(0.0);
                    
                    return Ok(StockInfo {
                        name,
                        code,
                        current_price,
                        yesterday_close,
                        open_price,
                        volume,
                        turnover,
                        high_price,
                        low_price,
                        change_amount,
                        change_percent,
                    });
                }
            }
        }

        Err(StockCalcError::ApiResponseError(
            format!("无法解析股票信息，响应: {}", response_text)
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;

    #[tokio::test]
    async fn test_parse_stock_price() {
        let config = AppConfig::default();
        let api = StockApi::new(config).unwrap();
        
        let test_response = r#"v_sh513500="1~标普 500ETF~513500~2.172~2.202~2.170~1791382~833022~958360~2.171~3820~2.170~3471~2.169~15568~2.168~5066~2.167~2027~2.172~606~2.173~2669~2.174~1992~2.175~3548~2.176~2355~~20250801161450~-0.030~-1.36~2.183~2.169~2.172/1791382/389956010~1791382~38996~1.83~~~2.183~2.169~0.64~212.70~212.70~0.00~2.422~1.982~1.24~18782~2.177~~~~~~38995.6010~0.0000~0~ ~ETF~0.46~0.37~~~~2.501~1.695~1.35~2.21~11.96~9792638600~9792638600~45.67~7.05~9792638600~1.41~2.1418~10.14~0.00~2.1416~CNY~0~**\_D**F\_\_Y~2.181~-16971""#;
        
        let price = api.parse_stock_price(test_response).unwrap();
        assert_eq!(price, 2.172);
    }
} 