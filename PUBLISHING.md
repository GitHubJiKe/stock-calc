# 发布指南

本指南将帮助您将股票收益计算器发布到 crates.io，让用户可以通过 Cargo 安装。

## 1. 发布到 crates.io

### 1.1 准备工作

1. **注册 crates.io 账户**

    ```bash
    # 访问 https://crates.io/settings/tokens
    # 创建新的 API token
    ```

2. **登录 Cargo**

    ```bash
    cargo login <your-api-token>
    ```

3. **验证包信息**
    ```bash
    cargo package
    ```

### 1.2 发布步骤

1. **检查包信息**

    ```bash
    cargo check
    cargo test
    cargo build --release
    ```

2. **发布到 crates.io**

    ```bash
    cargo publish
    ```

3. **验证发布**
    ```bash
    cargo search stock-calc
    ```

### 1.3 发布后用户安装

用户可以通过以下方式安装：

```bash
# 从 crates.io 安装
cargo install stock-calc

# 使用
stock-calc --help
```

## 2. 自动化发布流程

### 2.1 创建 GitHub Actions

创建 `.github/workflows/release.yml`：

```yaml
name: Release

on:
    push:
        tags:
            - "v*"

jobs:
    build:
        runs-on: ${{ matrix.os }}
        strategy:
            matrix:
                os: [ubuntu-latest, macos-latest]
                target:
                    - x86_64-unknown-linux-gnu
                    - x86_64-apple-darwin

        steps:
            - uses: actions/checkout@v4

            - name: Install Rust
              uses: actions-rs/toolchain@v1
              with:
                  toolchain: stable
                  target: ${{ matrix.target }}

            - name: Build
              run: cargo build --release --target ${{ matrix.target }}

            - name: Create release
              uses: softprops/action-gh-release@v1
              with:
                  files: |
                      target/${{ matrix.target }}/release/stock-calc
                  draft: false
                  prerelease: false
              env:
                  GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

    publish:
        needs: build
        runs-on: ubuntu-latest
        steps:
            - uses: actions/checkout@v4

            - name: Install Rust
              uses: actions-rs/toolchain@v1
              with:
                  toolchain: stable

            - name: Publish to crates.io
              run: cargo publish
              env:
                  CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

### 2.2 创建发布脚本

创建 `scripts/release.sh`：

```bash
#!/bin/bash

# 获取版本号
VERSION=$(grep '^version = ' Cargo.toml | cut -d '"' -f2)

# 构建
cargo build --release

# 创建发布
git tag v$VERSION
git push origin v$VERSION

# 发布到 crates.io
cargo publish

echo "Released version $VERSION"
```

## 3. 文档和营销

### 3.1 更新 README

确保 README 包含：

-   安装说明
-   使用示例
-   功能特性
-   贡献指南

### 3.2 创建项目网站

可以考虑创建简单的项目网站，展示：

-   功能介绍
-   安装指南
-   使用示例
-   更新日志

## 4. 维护和更新

### 4.1 版本管理

使用语义化版本控制：

-   MAJOR.MINOR.PATCH
-   例如：1.0.0, 1.0.1, 1.1.0

### 4.2 更新流程

1. **更新版本号**

    ```bash
    # 在 Cargo.toml 中更新版本
    version = "1.0.1"
    ```

2. **提交更改**

    ```bash
    git add .
    git commit -m "Bump version to 1.0.1"
    git tag v1.0.1
    git push origin main --tags
    ```

3. **发布**
    ```bash
    cargo publish
    ```

## 5. 用户支持

### 5.1 创建 Issues 模板

创建 `.github/ISSUE_TEMPLATE/bug_report.md`：

```markdown
## Bug 描述

## 重现步骤

## 预期行为

## 实际行为

## 环境信息

-   OS:
-   版本:
-   安装方式:
```

### 5.2 创建 Pull Request 模板

创建 `.github/PULL_REQUEST_TEMPLATE.md`：

```markdown
## 更改描述

## 类型

-   [ ] Bug 修复
-   [ ] 新功能
-   [ ] 文档更新
-   [ ] 其他

## 测试

-   [ ] 单元测试通过
-   [ ] 手动测试通过
```

## 6. 监控和反馈

### 6.1 使用统计

-   监控 crates.io 下载量
-   监控 GitHub 星标和 Fork
-   收集用户反馈

### 6.2 社区建设

-   回复 Issues 和 Pull Requests
-   参与相关社区讨论
-   分享使用案例和教程

## 总结

通过以上步骤，您可以将股票收益计算器发布到 crates.io，让更多用户受益。记住：

1. **质量第一**: 确保代码质量和测试覆盖
2. **文档完善**: 提供清晰的使用文档
3. **持续维护**: 及时响应用户反馈
4. **社区建设**: 积极参与开源社区

这样可以让您的工具被更多人使用和贡献！
