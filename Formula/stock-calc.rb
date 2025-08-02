class StockCalc < Formula
  desc "股票收益计算器 - 命令行工具"
  homepage "https://github.com/GitHubJiKe/stock-calc"
  version "1.0.0"

  # 根据操作系统选择不同的下载链接
  if OS.mac?
    # macOS版本的下载链接
    url "https://github.com/GitHubJiKe/stock-calc/releases/download/v1.0.0/stock-calc-x86_64-apple-darwin"
    # 需要替换为实际的SHA256哈希值
    sha256 "your-sha256-hash-here"
  elsif OS.linux?
    # Linux版本的下载链接
    url "https://github.com/GitHubJiKe/stock-calc/releases/download/v1.0.0/stock-calc-x86_64-unknown-linux-gnu"
    # 需要替换为实际的SHA256哈希值
    sha256 "your-sha256-hash-here"
  end

  # 安装方法：将二进制文件安装到bin目录
  def install
    bin.install "stock-calc"
  end

  # 测试方法：验证安装是否成功
  test do
    system "#{bin}/stock-calc", "--version"
  end
end 