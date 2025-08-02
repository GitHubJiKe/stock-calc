use clap::{Parser, Subcommand};
use crate::error::Result;
use crate::models::StockData;
use crate::config::AppConfig;
use chrono::Utc;

#[derive(Parser)]
#[command(name = "stock-calc")]
#[command(about = "股票收益计算器 - 命令行工具")]
#[command(version = "1.0.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 计算股票收益
    Calculate {
        /// 股票代码
        #[arg(short, long)]
        code: String,

        /// 持有数量
        #[arg(short, long)]
        quantity: f64,

        /// 购买均价
        #[arg(short, long)]
        avg_price: f64,

        /// 预期收益
        #[arg(short, long)]
        target_profit: f64,

        /// 最大亏损
        #[arg(short, long)]
        max_loss: f64,

        /// 是否保存到数据库
        #[arg(short, long, default_value = "false")]
        save: bool,
    },

    /// 实时监控股票
    Monitor {
        /// 股票代码
        #[arg(short, long)]
        code: String,

        /// 监控间隔（秒）
        #[arg(short, long, default_value = "60")]
        interval: u64,

        /// 最大重试次数
        #[arg(short, long, default_value = "3")]
        retry: u32,
    },

    /// 查看历史数据
    List {
        /// 显示详细信息
        #[arg(short, long, default_value = "false")]
        detailed: bool,
    },

    /// 删除股票数据
    Remove {
        /// 股票代码
        #[arg(short, long)]
        code: String,
    },

    /// 交互式模式
    Interactive,

    /// 配置管理
    Config {
        #[command(subcommand)]
        subcommand: ConfigSubcommand,
    },
}

#[derive(Subcommand)]
pub enum ConfigSubcommand {
    /// 显示当前配置
    Show,
    
    /// 重置为默认配置
    Reset,
    
    /// 编辑配置
    Edit,
}

pub struct InteractiveMode;

impl InteractiveMode {
    pub async fn run() -> Result<()> {
        println!("🎯 股票收益计算器 - 交互式模式");
        println!("{}", "━".repeat(50));

        let code = Self::prompt_input("请输入股票代码")?;
        let quantity = Self::prompt_number("请输入持有数量")?;
        let avg_price = Self::prompt_number("请输入购买均价")?;
        let target_profit = Self::prompt_number("请输入预期收益")?;
        let max_loss = Self::prompt_number("请输入最大亏损")?;

        let stock_data = StockData {
            code,
            quantity,
            avg_price,
            target_profit,
            max_loss,
            last_updated: Utc::now(),
        };

        // 验证输入
        crate::calculator::StockCalculator::validate_input(
            &stock_data.code,
            stock_data.quantity,
            stock_data.avg_price,
            stock_data.target_profit,
            stock_data.max_loss,
        )?;

        // 获取实时价格
        let config = AppConfig::load().unwrap_or_default();
        let api = crate::api::StockApi::new(config.clone())?;
        let current_price = api.fetch_stock_price(&stock_data.code).await?;

        // 计算分析
        let analysis = crate::calculator::StockCalculator::calculate_analysis(&stock_data, current_price);

        // 显示结果
        let notifier = crate::notification::Notifier::new(config.clone());
        notifier.print_analysis(&analysis);

        // 询问是否保存
        let save = Self::prompt_yes_no("是否保存到数据库")?;
        if save {
            let storage = crate::storage::Storage::new(config);
            storage.add_stock(stock_data)?;
            println!("✅ 数据已保存");
        }

        Ok(())
    }

    fn prompt_input(prompt: &str) -> Result<String> {
        use std::io::{self, Write};
        
        print!("{}: ", prompt);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        Ok(input.trim().to_string())
    }

    fn prompt_number(prompt: &str) -> Result<f64> {
        loop {
            let input = Self::prompt_input(prompt)?;
            match input.parse::<f64>() {
                Ok(num) if num > 0.0 => return Ok(num),
                _ => println!("❌ 请输入有效的正数"),
            }
        }
    }

    fn prompt_yes_no(prompt: &str) -> Result<bool> {
        loop {
            let input = Self::prompt_input(&format!("{} (y/n)", prompt))?;
            match input.to_lowercase().as_str() {
                "y" | "yes" | "是" => return Ok(true),
                "n" | "no" | "否" => return Ok(false),
                _ => println!("❌ 请输入 y/n 或 是/否"),
            }
        }
    }
}

pub fn print_help() {
    println!("股票收益计算器 v1.0.0");
    println!();
    println!("使用方法:");
    println!("  stock-calc <命令> [选项]");
    println!();
    println!("命令:");
    println!("  calculate    计算股票收益");
    println!("  monitor      实时监控股票");
    println!("  list         查看历史数据");
    println!("  remove       删除股票数据");
    println!("  interactive  交互式模式");
    println!("  config       配置管理");
    println!();
    println!("示例:");
    println!("  # 计算单次收益");
    println!("  stock-calc calculate --code 000001 --quantity 1000 --avg-price 15.5 --target-profit 5000 --max-loss 2000");
    println!();
    println!("  # 实时监控");
    println!("  stock-calc monitor --code 000001 --interval 60");
    println!();
    println!("  # 交互式模式");
    println!("  stock-calc interactive");
    println!();
    println!("  # 查看帮助");
    println!("  stock-calc --help");
} 