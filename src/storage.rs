use crate::error::Result;
use crate::models::{StockDatabase, StockData};
use crate::config::AppConfig;
use serde_json;
use std::fs;
use std::path::Path;

pub struct Storage {
    config: AppConfig,
}

impl Storage {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn load_database(&self) -> Result<StockDatabase> {
        let data_path = AppConfig::get_data_path()?;
        
        if !Path::new(&data_path).exists() {
            return Ok(StockDatabase::new());
        }

        let content = fs::read_to_string(&data_path)?;
        let database: StockDatabase = serde_json::from_str(&content)?;
        Ok(database)
    }

    pub fn save_database(&self, database: &StockDatabase) -> Result<()> {
        let data_path = AppConfig::get_data_path()?;
        
        // 确保目录存在
        if let Some(parent) = data_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(database)?;
        fs::write(&data_path, json)?;
        
        log::info!("数据已保存到: {:?}", data_path);
        Ok(())
    }

    pub fn add_stock(&self, stock: StockData) -> Result<()> {
        let mut database = self.load_database()?;
        database.add_stock(stock);
        self.save_database(&database)
    }

    pub fn remove_stock(&self, code: &str) -> Result<Option<StockData>> {
        let mut database = self.load_database()?;
        let removed = database.remove_stock(code);
        
        if removed.is_some() {
            self.save_database(&database)?;
        }
        
        Ok(removed)
    }

    pub fn get_stock(&self, code: &str) -> Result<Option<StockData>> {
        let database = self.load_database()?;
        Ok(database.get_stock(code).cloned())
    }

    pub fn list_stocks(&self) -> Result<Vec<StockData>> {
        let database = self.load_database()?;
        Ok(database.list_stocks().into_iter().cloned().collect())
    }

    pub fn update_stock(&self, stock: StockData) -> Result<()> {
        let mut database = self.load_database()?;
        database.add_stock(stock);
        self.save_database(&database)
    }

    pub fn backup_database(&self) -> Result<()> {
        let data_path = AppConfig::get_data_path()?;
        let backup_path = data_path.with_extension("json.backup");
        
        if Path::new(&data_path).exists() {
            fs::copy(&data_path, &backup_path)?;
            log::info!("数据库已备份到: {:?}", backup_path);
        }
        
        Ok(())
    }

    pub fn restore_database(&self) -> Result<()> {
        let data_path = AppConfig::get_data_path()?;
        let backup_path = data_path.with_extension("json.backup");
        
        if Path::new(&backup_path).exists() {
            fs::copy(&backup_path, &data_path)?;
            log::info!("数据库已从备份恢复");
        } else {
            return Err(crate::error::StockCalcError::StorageError(
                "备份文件不存在".to_string()
            ));
        }
        
        Ok(())
    }

    pub fn clear_database(&self) -> Result<()> {
        let database = StockDatabase::new();
        self.save_database(&database)
    }

    pub fn get_database_info(&self) -> Result<DatabaseInfo> {
        let data_path = AppConfig::get_data_path()?;
        let database = self.load_database()?;
        
        let file_size = if Path::new(&data_path).exists() {
            fs::metadata(&data_path)?.len()
        } else {
            0
        };

        Ok(DatabaseInfo {
            total_stocks: database.stocks.len(),
            file_size,
            data_path: data_path.to_string_lossy().to_string(),
        })
    }
}

#[derive(Debug)]
pub struct DatabaseInfo {
    pub total_stocks: usize,
    pub file_size: u64,
    pub data_path: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::StockData;
    use chrono::Utc;

    #[test]
    fn test_storage_operations() {
        let config = AppConfig::default();
        let storage = Storage::new(config);
        
        // 测试添加股票
        let stock = StockData {
            code: "000001".to_string(),
            quantity: 1000.0,
            avg_price: 15.5,
            target_profit: 5000.0,
            max_loss: 2000.0,
            last_updated: Utc::now(),
        };

        assert!(storage.add_stock(stock.clone()).is_ok());
        
        // 测试获取股票
        let retrieved = storage.get_stock("000001").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().code, "000001");
        
        // 测试删除股票
        let removed = storage.remove_stock("000001").unwrap();
        assert!(removed.is_some());
        
        // 验证已删除
        let retrieved = storage.get_stock("000001").unwrap();
        assert!(retrieved.is_none());
    }
} 