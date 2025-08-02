# 股票收益计算器

一个功能强大的 Rust 命令行工具，用于计算和监控股票投资收益。

## 功能特性

-   📊 **收益计算**: 精确计算目标价格、止损价格、当前盈亏等
-   🔄 **实时监控**: 支持实时监控股票价格变化
-   💰 **风险评估**: 提供风险收益比和投资规模分析
-   🔔 **智能提醒**: 达到目标或止损时自动通知
-   💾 **数据持久化**: 本地保存股票数据
-   🎨 **美观界面**: 彩色输出和进度显示
-   ⚙️ **灵活配置**: 支持自定义配置

## 安装

### 使用 Cargo 安装 (推荐)

```bash
# 从 crates.io 安装
cargo install stock-calc

# 使用
stock-calc --help
```

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/GitHubJiKe/stock-calc.git
cd stock-calc

# 编译
cargo build --release

# 安装到系统
cargo install --path .
```

### 下载预编译二进制文件

访问 [GitHub Releases](https://github.com/GitHubJiKe/stock-calc/releases) 下载适合您系统的预编译二进制文件。

## 使用方法

### 基础命令

```bash
# 计算单次收益
stock-calc calculate --code 000001 --quantity 1000 --avg-price 15.5 --target-profit 5000 --max-loss 2000

# 实时监控
stock-calc monitor --code 000001 --interval 60

# 查看历史数据
stock-calc list

# 删除股票数据
stock-calc remove --code 000001

# 交互式模式
stock-calc interactive
```

### 交互式模式

```bash
stock-calc interactive
```

交互式模式会引导您输入股票信息：

```
🎯 股票收益计算器 - 交互式模式
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
请输入股票代码: 000001
请输入持有数量: 1000
请输入购买均价: 15.5
请输入预期收益: 5000
请输入最大亏损: 2000
```

### 配置管理

```bash
# 显示当前配置
stock-calc config show

# 重置为默认配置
stock-calc config reset

# 编辑配置
stock-calc config edit
```

## 配置说明

配置文件位置: `~/.config/stock-calc/config.toml`

```toml
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

## 输出示例

### 收益分析

```
📊 股票收益分析: 000001
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💰 投资信息
   持有数量: 1,000 股
   购买均价: ¥15.500
   投资总额: ¥15,500.000

📈 盈利目标
   目标售价: ¥20.500 (+32.26%)
   预期收益: ¥5,000.000
   距离目标: +2.44%

📉 止损目标
   止损价格: ¥13.500 (-12.90%)
   最大亏损: ¥2,000.000
   安全空间: +11.11%

📊 当前状态
   当前价格: ¥16.200 (+4.52%)
   当前盈亏: +¥700.000
   盈亏比例: +4.52%

⚠️  风险提示
   风险收益比: 2.5:1 (一般)
   建议: 继续持有观察
```

### 实时监控

```
🔄 实时监控: 000001 (每60秒更新)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⏰ 2024-01-01 12:34:56
💰 当前价格: ¥16.200 (+4.52%)
📊 距离目标: +2.44% | 距离止损: +11.11%
```

## 数据存储

股票数据保存在: `~/.config/stock-calc/data.json`

```json
{
    "stocks": {
        "000001": {
            "code": "000001",
            "quantity": 1000.0,
            "avg_price": 15.5,
            "target_profit": 5000.0,
            "max_loss": 2000.0,
            "last_updated": "2024-01-01T12:00:00Z"
        }
    }
}
```

## 环境变量

```bash
# 配置文件路径
STOCK_CALC_CONFIG_PATH=/path/to/config

# 数据文件路径
STOCK_CALC_DATA_PATH=/path/to/data

# 日志级别
STOCK_CALC_LOG_LEVEL=debug
```

## 开发

### 项目结构

```
stock-calc/
├── src/
│   ├── main.rs          # 主程序入口
│   ├── cli.rs           # 命令行界面
│   ├── calculator.rs     # 收益计算逻辑
│   ├── api.rs           # API请求和解析
│   ├── storage.rs       # 数据持久化
│   ├── notification.rs  # 提醒功能
│   ├── config.rs        # 配置管理
│   ├── models.rs        # 数据模型
│   └── error.rs         # 错误处理
├── tests/               # 测试文件
├── examples/            # 示例代码
└── Cargo.toml          # 项目配置
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_calculate_analysis

# 运行集成测试
cargo test --test integration_tests
```

### 代码格式化

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy
```

## 贡献

欢迎提交 Issue 和 Pull Request！

### 开发指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 更新日志

### v1.0.0

-   初始版本发布
-   基础收益计算功能
-   实时监控功能
-   数据持久化
-   系统通知和声音提醒
-   交互式模式
-   配置管理

## 致谢

-   [腾讯股票 API](http://sqt.gtimg.cn/) - 提供股票数据
-   [Rust 社区](https://www.rust-lang.org/) - 优秀的编程语言
-   所有贡献者和用户

---

**注意**: 本工具仅供学习和研究使用，不构成投资建议。投资有风险，入市需谨慎。
