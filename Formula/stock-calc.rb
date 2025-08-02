class StockCalc < Formula
  desc "股票收益计算器 - 命令行工具"
  homepage "https://github.com/GitHubJiKe/stock-calc"
  version "1.0.0"

  if OS.mac?
    url "https://github.com/GitHubJiKe/stock-calc/releases/download/v1.0.0/stock-calc-x86_64-apple-darwin"
    sha256 "your-sha256-hash-here"
  elsif OS.linux?
    url "https://github.com/GitHubJiKe/stock-calc/releases/download/v1.0.0/stock-calc-x86_64-unknown-linux-gnu"
    sha256 "your-sha256-hash-here"
  end

  def install
    bin.install "stock-calc"
  end

  test do
    system "#{bin}/stock-calc", "--version"
  end
end 