#!/bin/bash

set -e

# 获取版本号
VERSION=$(grep '^version = ' Cargo.toml | cut -d '"' -f2)

echo "🚀 准备发布版本 $VERSION"

# 检查是否有未提交的更改
if [[ -n $(git status --porcelain) ]]; then
    echo "❌ 有未提交的更改，请先提交所有更改"
    exit 1
fi

# 运行测试
echo "🧪 运行测试..."
cargo test

# 构建
echo "🔨 构建项目..."
cargo build --release

# 检查是否已经存在该版本的标签
if git tag | grep -q "v$VERSION"; then
    echo "❌ 版本 v$VERSION 已经存在"
    exit 1
fi

# 创建标签
echo "🏷️  创建标签 v$VERSION"
git tag v$VERSION

# 推送标签
echo "📤 推送标签到远程仓库..."
git push origin v$VERSION

echo "✅ 版本 $VERSION 已发布到 GitHub"
echo "📦 接下来可以运行: cargo publish" 