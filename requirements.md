# 股票收益计算器 - Rust 命令行工具需求文档

## 项目概述

将现有的 Web 版股票收益计算器迁移为 Rust 命令行工具，保持核心功能的同时优化性能和用户体验。

## 核心功能需求

### 1. 股票收益计算功能

#### 1.1 基础计算

-   **目标价格计算**: `目标售价 = 购买均价 + (预期收益 / 持有数量)`
-   **止损价格计算**: `止损价格 = 购买均价 - (最大亏损 / 持有数量)`
-   **当前盈亏计算**: `当前盈亏 = (当前价格 - 购买均价) × 持有数量`
-   **盈亏比例计算**: `盈亏比例 = ((当前价格 - 购买均价) / 购买均价) × 100%`

#### 1.2 风险评估

-   **风险收益比**: `风险收益比 = 预期收益 / 最大亏损`
-   **投资规模评估**: 根据总投资金额分类（小额<1 万、中等 1-10 万、大额>10 万）
-   **价格距离分析**: `距离目标 = (目标价格 - 当前价格) / 当前价格 × 100%`

### 2. 实时股价获取功能

#### 2.1 API 集成

-   **数据源**: 腾讯股票 API (`http://sqt.gtimg.cn/utf8/q={stock_code}`)
-   **响应格式**: `v_{stock_code}="数据字段"`
-   **价格提取**: 解析第 4 个字段作为当前股价
-   **错误处理**: 网络错误、数据格式错误、股票代码无效等

#### 2.2 数据解析

```rust
// 示例响应数据
"v_sh513500=\"1~标普 500ETF~513500~2.172~2.202~2.170~1791382~833022~958360~2.171~3820~2.170~3471~2.169~15568~2.168~5066~2.167~2027~2.172~606~2.173~2669~2.174~1992~2.175~3548~2.176~2355~~20250801161450~-0.030~-1.36~2.183~2.169~2.172/1791382/389956010~1791382~38996~1.83~~~2.183~2.169~0.64~212.70~212.70~0.00~2.422~1.982~1.24~18782~2.177~~~~~~38995.6010~0.0000~0~ ~ETF~0.46~0.37~~~~2.501~1.695~1.35~2.21~11.96~9792638600~9792638600~45.67~7.05~9792638600~1.41~2.1418~10.14~0.00~2.1416~CNY~0~**\_D**F\_\_Y~2.181~-16971\""
```

### 3. 数据持久化功能

#### 3.1 本地存储

-   **存储格式**: JSON 文件
-   **存储位置**: 用户主目录下的配置文件
-   **数据结构**:

```json
{
    "stocks": {
        "000001": {
            "quantity": 1000,
            "avg_price": 15.5,
            "target_profit": 5000,
            "max_loss": 2000,
            "last_updated": "2024-01-01T12:00:00Z"
        }
    }
}
```

#### 3.2 数据管理

-   **自动保存**: 计算完成后自动保存
-   **数据加载**: 启动时自动加载历史数据
-   **数据验证**: 验证数据完整性和有效性

### 4. 提醒功能

#### 4.1 提醒触发条件

-   **盈利目标**: 当前价格 ≥ 目标售价
-   **止损触发**: 当前价格 ≤ 止损价格

#### 4.2 提醒方式

-   **终端通知**: 彩色输出、特殊字符、声音提示
-   **系统通知**: 使用系统原生通知（macOS、Linux、Windows）
-   **声音提示**: 生成不同频率的提示音
-   **日志记录**: 详细的操作日志

### 5. 命令行界面

#### 5.1 基础命令

```bash
# 计算单次收益
stock-calc calculate --code 000001 --quantity 1000 --avg-price 15.5 --target-profit 5000 --max-loss 2000

# 实时监控
stock-calc monitor --code 000001 --interval 60

# 查看历史数据
stock-calc list

# 删除股票数据
stock-calc remove --code 000001
```

#### 5.2 交互式模式

```bash
# 交互式配置
stock-calc interactive

# 问答式输入
> 请输入股票代码: 000001
> 请输入持有数量: 1000
> 请输入购买均价: 15.5
> 请输入预期收益: 5000
> 请输入最大亏损: 2000
```

### 6. 配置管理

#### 6.1 配置文件

```toml
# ~/.config/stock-calc/config.toml
[general]
default_interval = 60
enable_notifications = true
enable_sound = true
log_level = "info"

[api]
timeout = 8
retry_count = 3
user_agent = "StockCalc/1.0"

[display]
color_output = true
show_progress = true
```

#### 6.2 环境变量

```bash
STOCK_CALC_CONFIG_PATH=/path/to/config
STOCK_CALC_DATA_PATH=/path/to/data
STOCK_CALC_LOG_LEVEL=debug
```

## 技术实现需求

### 1. 项目结构

```
stock-calc/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli.rs          # 命令行参数处理
│   ├── calculator.rs    # 收益计算逻辑
│   ├── api.rs          # API请求和解析
│   ├── storage.rs      # 数据持久化
│   ├── notification.rs # 提醒功能
│   ├── config.rs       # 配置管理
│   └── utils.rs        # 工具函数
├── tests/
└── examples/
```

### 2. 依赖库选择

#### 2.1 核心依赖

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
clap = { version = "4.0", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

#### 2.2 可选依赖

```toml
[dependencies]
notify-rust = "4.0"     # 系统通知
crossterm = "0.27"      # 终端控制
tui = "0.19"           # 终端UI
config = "0.13"        # 配置管理
log = "0.4"            # 日志
env_logger = "0.10"    # 日志实现
```

### 3. 错误处理

#### 3.1 错误类型定义

```rust
#[derive(Debug, thiserror::Error)]
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
}
```

### 4. 异步处理

#### 4.1 实时监控

```rust
pub async fn monitor_stock(
    stock_code: String,
    interval: Duration,
    config: Config,
) -> Result<(), StockCalcError> {
    let mut interval = tokio::time::interval(interval);

    loop {
        interval.tick().await;

        let price = fetch_stock_price(&stock_code).await?;
        let analysis = calculate_profit_analysis(&stock_code, price).await?;

        display_analysis(&analysis);
        check_alerts(&analysis, &config).await?;
    }
}
```

### 5. 数据验证

#### 5.1 输入验证

```rust
pub fn validate_stock_input(
    code: &str,
    quantity: f64,
    avg_price: f64,
    target_profit: f64,
    max_loss: f64,
) -> Result<(), ValidationError> {
    // 股票代码格式验证
    if !code.chars().all(|c| c.is_alphanumeric()) {
        return Err(ValidationError::InvalidStockCode);
    }

    // 数值验证
    if quantity <= 0.0 || avg_price <= 0.0 || target_profit <= 0.0 || max_loss <= 0.0 {
        return Err(ValidationError::InvalidNumber);
    }

    Ok(())
}
```

## 用户体验需求

### 1. 输出格式

#### 1.1 计算结果展示

```
📊 股票收益分析: 000001
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💰 投资信息
   持有数量: 1,000 股
   购买均价: ¥15.500
   投资总额: ¥15,500.00

📈 盈利目标
   目标售价: ¥20.500 (+32.26%)
   预期收益: ¥5,000.00
   距离目标: +2.44%

📉 止损目标
   止损价格: ¥13.500 (-12.90%)
   最大亏损: ¥2,000.00
   安全空间: +11.11%

📊 当前状态
   当前价格: ¥16.200 (+4.52%)
   当前盈亏: +¥700.00
   盈亏比例: +4.52%

⚠️  风险提示
   风险收益比: 2.5:1 (一般)
   建议: 考虑分批卖出
```

#### 1.2 实时监控输出

```
🔄 实时监控: 000001 (每60秒更新)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⏰ 2024-01-01 12:34:56
💰 当前价格: ¥16.200 (+4.52%)
📊 距离目标: +2.44% | 距离止损: +11.11%

⏰ 2024-01-01 12:35:56
💰 当前价格: ¥16.350 (+5.48%)
📊 距离目标: +1.83% | 距离止损: +12.22%

🎉 目标达成! 当前价格 ¥20.500 已达到目标售价!
```

### 2. 交互体验

#### 2.1 进度显示

```
正在获取实时数据... [██████████] 100%
正在计算收益分析... [██████████] 100%
正在保存数据... [██████████] 100%
```

#### 2.2 错误提示

```
❌ 错误: 网络连接失败
   请检查网络连接后重试

💡 提示: 可以尝试以下命令
   stock-calc monitor --code 000001 --retry 3
```

### 3. 帮助信息

#### 3.1 命令帮助

```bash
$ stock-calc --help
股票收益计算器 v1.0.0

USAGE:
    stock-calc <SUBCOMMAND>

SUBCOMMANDS:
    calculate    计算股票收益
    monitor      实时监控股票
    list         查看历史数据
    remove       删除股票数据
    config       配置管理
    help         显示帮助信息

OPTIONS:
    -h, --help       显示帮助信息
    -V, --version    显示版本信息
```

## 性能要求

### 1. 响应时间

-   **API 请求**: < 3 秒
-   **计算处理**: < 100ms
-   **数据保存**: < 50ms

### 2. 资源使用

-   **内存占用**: < 50MB
-   **CPU 使用**: < 5% (空闲时)
-   **网络流量**: 最小化请求频率

### 3. 并发处理

-   **多股票监控**: 支持同时监控多个股票
-   **异步处理**: 非阻塞的 API 请求
-   **错误恢复**: 自动重试机制

## 测试要求

### 1. 单元测试

-   计算逻辑测试
-   API 解析测试
-   数据验证测试

### 2. 集成测试

-   端到端功能测试
-   错误处理测试
-   性能基准测试

### 3. 用户测试

-   命令行体验测试
-   配置管理测试
-   提醒功能测试

## 部署要求

### 1. 跨平台支持

-   **macOS**: 10.15+
-   **Linux**: Ubuntu 18.04+, CentOS 7+
-   **Windows**: Windows 10+

### 2. 安装方式

```bash
# Cargo安装
cargo install stock-calc

# 二进制下载
curl -L https://github.com/user/stock-calc/releases/latest/download/stock-calc-x86_64-unknown-linux-gnu.tar.gz | tar xz

# 包管理器
# macOS: brew install stock-calc
# Linux: snap install stock-calc
```

### 3. 配置迁移

-   支持从 Web 版本导入数据
-   提供数据导出功能
-   配置文件自动生成

## 开发优先级

### Phase 1: 核心功能

1. 基础计算功能
2. API 集成
3. 命令行界面
4. 数据持久化

### Phase 2: 高级功能

1. 实时监控
2. 提醒功能
3. 配置管理
4. 错误处理

### Phase 3: 优化完善

1. 性能优化
2. 用户体验
3. 测试覆盖
4. 文档完善

---

**文档版本**: 1.0  
**创建日期**: 2024 年 1 月  
**维护者**: [Your Name]
