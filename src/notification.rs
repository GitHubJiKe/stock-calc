use crate::error::Result;
use crate::models::{StockAnalysis, StockStatus};
use crate::config::AppConfig;
use colored::*;
use notify_rust::Notification;


pub struct Notifier {
    config: AppConfig,
}

impl Notifier {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub async fn check_alerts(&self, analysis: &StockAnalysis) -> Result<()> {
        match analysis.status {
            StockStatus::AtTarget => {
                self.send_target_alert(analysis).await?;
            }
            StockStatus::AtStopLoss => {
                self.send_stop_loss_alert(analysis).await?;
            }
            StockStatus::NearTarget => {
                self.send_near_target_alert(analysis).await?;
            }
            StockStatus::NearStopLoss => {
                self.send_near_stop_loss_alert(analysis).await?;
            }
            _ => {}
        }
        Ok(())
    }

    async fn send_target_alert(&self, analysis: &StockAnalysis) -> Result<()> {
        let title = "🎉 目标达成!";
        let body = format!(
            "股票 {} 已达到目标价格 ¥{:.2}",
            analysis.code, analysis.current_price
        );

        self.send_system_notification(title, &body).await?;
        self.play_sound_alert("success").await?;
        self.print_colored_alert(title, &body, "green");
        
        Ok(())
    }

    async fn send_stop_loss_alert(&self, analysis: &StockAnalysis) -> Result<()> {
        let title = "⚠️ 止损触发!";
        let body = format!(
            "股票 {} 已达到止损价格 ¥{:.2}",
            analysis.code, analysis.current_price
        );

        self.send_system_notification(title, &body).await?;
        self.play_sound_alert("warning").await?;
        self.print_colored_alert(title, &body, "red");
        
        Ok(())
    }

    async fn send_near_target_alert(&self, analysis: &StockAnalysis) -> Result<()> {
        let title = "🎯 接近目标!";
        let body = format!(
            "股票 {} 距离目标价格还有 {:.2}%",
            analysis.code, analysis.distance_to_target
        );

        self.send_system_notification(title, &body).await?;
        self.print_colored_alert(title, &body, "yellow");
        
        Ok(())
    }

    async fn send_near_stop_loss_alert(&self, analysis: &StockAnalysis) -> Result<()> {
        let title = "🚨 接近止损!";
        let body = format!(
            "股票 {} 距离止损价格还有 {:.2}%",
            analysis.code, analysis.distance_to_stop_loss
        );

        self.send_system_notification(title, &body).await?;
        self.print_colored_alert(title, &body, "red");
        
        Ok(())
    }

    async fn send_system_notification(&self, title: &str, body: &str) -> Result<()> {
        if !self.config.general.enable_notifications {
            return Ok(());
        }

        let notification = Notification::new()
            .summary(title)
            .body(body)
            .timeout(5000)
            .show();

        match notification {
            Ok(_) => log::info!("系统通知已发送: {}", title),
            Err(e) => {
                log::warn!("发送系统通知失败: {}", e);
                return Err(crate::error::StockCalcError::NotificationError(
                    e.to_string()
                ));
            }
        }

        Ok(())
    }

    async fn play_sound_alert(&self, alert_type: &str) -> Result<()> {
        if !self.config.general.enable_sound {
            return Ok(());
        }

        // 使用系统命令播放声音
        let sound_cmd = match alert_type {
            "success" => "afplay /System/Library/Sounds/Glass.aiff",
            "warning" => "afplay /System/Library/Sounds/Basso.aiff",
            _ => "afplay /System/Library/Sounds/Ping.aiff",
        };

        if let Err(e) = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(sound_cmd)
            .output()
            .await
        {
            log::warn!("播放声音失败: {}", e);
        }

        Ok(())
    }

    fn print_colored_alert(&self, title: &str, body: &str, color: &str) {
        if !self.config.display.color_output {
            println!("{}: {}", title, body);
            return;
        }

        let colored_title = match color {
            "green" => title.green().bold(),
            "red" => title.red().bold(),
            "yellow" => title.yellow().bold(),
            _ => title.white().bold(),
        };

        println!("{}: {}", colored_title, body);
    }

    pub fn print_analysis(&self, analysis: &StockAnalysis) {
        if !self.config.display.color_output {
            self.print_analysis_plain(analysis);
            return;
        }

        self.print_analysis_colored(analysis);
    }

    fn print_analysis_colored(&self, analysis: &StockAnalysis) {
        let status_emoji = crate::calculator::StockCalculator::get_status_emoji(&analysis.status);
        
        println!("\n{} 股票收益分析: {}", status_emoji, analysis.code.bold());
        println!("{}", "━".repeat(50));

        // 投资信息
        println!("💰 {}", "投资信息".green().bold());
        println!("   持有数量: {} 股", analysis.quantity);
        println!("   购买均价: {}", crate::calculator::StockCalculator::format_currency(analysis.avg_price));
        println!("   投资总额: {}", crate::calculator::StockCalculator::format_currency(analysis.investment_amount));

        // 盈利目标
        println!("\n📈 {}", "盈利目标".blue().bold());
        println!("   目标售价: {} ({})", 
            crate::calculator::StockCalculator::format_currency(analysis.target_price),
            format!("+{:.2}%", ((analysis.target_price - analysis.avg_price) / analysis.avg_price) * 100.0).green()
        );
        println!("   预期收益: {}", crate::calculator::StockCalculator::format_currency(analysis.target_profit));
        println!("   距离目标: {}", 
            if analysis.distance_to_target > 0.0 {
                format!("+{:.2}%", analysis.distance_to_target).green()
            } else {
                format!("{:.2}%", analysis.distance_to_target).red()
            }
        );

        // 止损目标
        println!("\n📉 {}", "止损目标".red().bold());
        println!("   止损价格: {} ({})", 
            crate::calculator::StockCalculator::format_currency(analysis.stop_loss_price),
            format!("-{:.2}%", ((analysis.avg_price - analysis.stop_loss_price) / analysis.avg_price) * 100.0).red()
        );
        println!("   最大亏损: {}", crate::calculator::StockCalculator::format_currency(analysis.max_loss));
        println!("   安全空间: {}", 
            if analysis.distance_to_stop_loss > 0.0 {
                format!("+{:.2}%", analysis.distance_to_stop_loss).green()
            } else {
                format!("{:.2}%", analysis.distance_to_stop_loss).red()
            }
        );

        // 当前状态
        println!("\n📊 {}", "当前状态".cyan().bold());
        println!("   当前价格: {} ({})", 
            crate::calculator::StockCalculator::format_currency(analysis.current_price),
            if analysis.profit_ratio > 0.0 {
                format!("+{:.2}%", analysis.profit_ratio).green()
            } else {
                format!("{:.2}%", analysis.profit_ratio).red()
            }
        );
        println!("   当前盈亏: {}", 
            if analysis.current_profit > 0.0 {
                format!("+{}", crate::calculator::StockCalculator::format_currency(analysis.current_profit)).green()
            } else {
                format!("{}", crate::calculator::StockCalculator::format_currency(analysis.current_profit)).red()
            }
        );
        println!("   盈亏比例: {}", 
            if analysis.profit_ratio > 0.0 {
                format!("+{:.2}%", analysis.profit_ratio).green()
            } else {
                format!("{:.2}%", analysis.profit_ratio).red()
            }
        );

        // 风险提示
        println!("\n⚠️  {}", "风险提示".yellow().bold());
        println!("   风险收益比: {:.1}:1 ({})", 
            analysis.risk_reward_ratio,
            crate::calculator::StockCalculator::get_risk_level(analysis.risk_reward_ratio)
        );
        println!("   投资规模: {}", 
            crate::calculator::StockCalculator::get_investment_scale_text(&analysis.investment_scale)
        );
        
        if analysis.profit_ratio > 20.0 {
            println!("   💡 建议: 考虑分批卖出");
        } else if analysis.profit_ratio < -10.0 {
            println!("   💡 建议: 考虑止损或补仓");
        } else {
            println!("   💡 建议: 继续持有观察");
        }
    }

    fn print_analysis_plain(&self, analysis: &StockAnalysis) {
        println!("\n📊 股票收益分析: {}", analysis.code);
        println!("{}", "━".repeat(50));

        println!("💰 投资信息");
        println!("   持有数量: {} 股", analysis.quantity);
        println!("   购买均价: ¥{:.3}", analysis.avg_price);
        println!("   投资总额: ¥{:.3}", analysis.investment_amount);

        println!("\n📈 盈利目标");
        println!("   目标售价: ¥{:.3} (+{:.2}%)", 
            analysis.target_price,
            ((analysis.target_price - analysis.avg_price) / analysis.avg_price) * 100.0
        );
        println!("   预期收益: ¥{:.3}", analysis.target_profit);
        println!("   距离目标: {:.2}%", analysis.distance_to_target);

        println!("\n📉 止损目标");
        println!("   止损价格: ¥{:.3} (-{:.2}%)", 
            analysis.stop_loss_price,
            ((analysis.avg_price - analysis.stop_loss_price) / analysis.avg_price) * 100.0
        );
        println!("   最大亏损: ¥{:.3}", analysis.max_loss);
        println!("   安全空间: {:.2}%", analysis.distance_to_stop_loss);

        println!("\n📊 当前状态");
        println!("   当前价格: ¥{:.3} ({:.2}%)", analysis.current_price, analysis.profit_ratio);
        println!("   当前盈亏: {:.3}", analysis.current_profit);
        println!("   盈亏比例: {:.2}%", analysis.profit_ratio);

        println!("\n⚠️  风险提示");
        println!("   风险收益比: {:.1}:1 ({})", 
            analysis.risk_reward_ratio,
            crate::calculator::StockCalculator::get_risk_level(analysis.risk_reward_ratio)
        );
        println!("   投资规模: {}", 
            crate::calculator::StockCalculator::get_investment_scale_text(&analysis.investment_scale)
        );
    }
} 