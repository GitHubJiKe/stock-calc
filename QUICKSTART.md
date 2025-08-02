# 快速开始

## 安装

选择最适合您的安装方式：

### 方式一：Cargo 安装 (推荐)

```bash
cargo install stock-calc
```

### 方式二：Homebrew 安装 (macOS)

```bash
brew install stock-calc
```

### 方式三：从源码编译

```bash
git clone https://github.com/GitHubJiKe/stock-calc.git
cd stock-calc
cargo build --release
cargo install --path .
```

## 快速使用

### 1. 计算单次收益

```bash
stock-calc calculate --code sh513500 --quantity 1000 --avg-price 2.175 --target-profit 500 --max-loss 200
```

### 2. 交互式模式

```bash
stock-calc interactive
```

### 3. 查看帮助

```bash
stock-calc --help
```

## 常用命令

| 命令          | 描述         | 示例                                                                  |
| ------------- | ------------ | --------------------------------------------------------------------- |
| `calculate`   | 计算股票收益 | `stock-calc calculate --code 000001 --quantity 1000 --avg-price 15.5` |
| `monitor`     | 实时监控     | `stock-calc monitor --code 000001 --interval 60`                      |
| `list`        | 查看历史数据 | `stock-calc list --detailed`                                          |
| `interactive` | 交互式模式   | `stock-calc interactive`                                              |
| `config`      | 配置管理     | `stock-calc config show`                                              |

## 配置

首次运行时会自动创建配置文件：`~/.config/stock-calc/config.toml`

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

## 数据存储

股票数据保存在：`~/.config/stock-calc/data.json`

## 支持的功能

-   ✅ 实时股票价格获取
-   ✅ 收益计算和分析
-   ✅ 风险评估
-   ✅ 数据持久化
-   ✅ 系统通知
-   ✅ 彩色输出
-   ✅ 交互式模式

## 获取帮助

-   查看帮助：`stock-calc --help`
-   查看命令帮助：`stock-calc <command> --help`
-   提交 Issue：https://github.com/GitHubJiKe/stock-calc/issues
-   查看文档：https://github.com/GitHubJiKe/stock-calc#readme
