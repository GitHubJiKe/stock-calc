use thiserror::Error;

#[derive(Debug, Error)]
pub enum StockCalcError {
    #[error("网络请求失败: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("数据解析失败: {0}")]
    ParseError(String),

    #[error("股票代码无效: {0}")]
    InvalidStockCode(String),

    #[error("配置错误: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("存储错误: {0}")]
    StorageError(String),

    #[error("输入验证失败: {0}")]
    ValidationError(String),

    #[error("文件操作失败: {0}")]
    FileError(#[from] std::io::Error),

    #[error("JSON序列化失败: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("通知发送失败: {0}")]
    NotificationError(String),

    #[error("API响应格式错误: {0}")]
    ApiResponseError(String),
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("股票代码格式无效")]
    InvalidStockCode,

    #[error("数值必须大于0")]
    InvalidNumber,

    #[error("持有数量必须为正数")]
    InvalidQuantity,

    #[error("价格必须为正数")]
    InvalidPrice,

    #[error("收益目标必须为正数")]
    InvalidTargetProfit,

    #[error("最大亏损必须为正数")]
    InvalidMaxLoss,
}

pub type Result<T> = std::result::Result<T, StockCalcError>; 