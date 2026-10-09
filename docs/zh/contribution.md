# 参与 DryDrop 贡献

感谢你有兴趣为 DryDrop 做贡献！本指南介绍如何报告问题、提交改动，以及在
发起 Pull Request 之前需要做哪些检查。

[English](../en/contribution.md) | 简体中文

## 参与方式

- **Bug 报告** —— 发现问题或行为异常？请开一个 issue，附上复现步骤、预期
  行为和实际行为。
- **功能建议** —— 请描述你想解决的问题本身，而不只是解决方案，这有助于我们
  评估它是否符合项目方向。
- **文档** —— 文档的修正和改进永远受欢迎。
- **代码** —— Bug 修复、重构和新功能，见下文。

## 开始之前

先搭建开发环境（完整指南见 [develop.md](../zh/develop.md)）：

```bash
mise install                 # 安装固定版本的工具链（Rust、dx、pnpm 等）
mise bootstrap               # rustup 目标 + Android NDK + pnpm install + Tailwind CSS
```

然后选择你想开发的应用，用对应的 `mise run *-dev` 任务启动
（`web-dev`、`macos-dev`、`ios-dev`、`android-dev`、`cli-dev`、`tui-dev`、
`server-dev`）。

## 提交之前

CI 运行的就是这些命令，请确保它们在本地全部通过：

```bash
mise run fmt      # cargo fmt --all -- --check
mise run check    # cargo check --workspace --all-features
mise run clippy   # cargo clippy --workspace --all-features -- -D warnings
mise run test     # cargo test --workspace --all-features
```

补充说明：

- 如果改动涉及任何被扫描的 UI 源码，请重新生成已提交的 Tailwind 资产 ——
  资产过期会导致 CI 失败（见 [develop.md](../zh/develop.md)）。
- 涉及 PostgreSQL / Redis / RabbitMQ 的集成测试需要先启动测试基础设施
  （`mise run infra-test-up`）。
- 请同时阅读 [develop.md](../zh/develop.md) 中的约定与注意事项 —— 里面列出了最容
  易导致 CI 失败的几类问题。

## Pull Request

- 保持每个 PR 聚焦于单一改动。较大的功能请先开 issue 讨论方案，再动手实现。
- 提交前先 rebase 到最新的 `main`。
- PR 描述请说明改动**为什么**需要，而不仅仅是做了什么。

## 提交信息

我们遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>(<scope>): <简短摘要>
```

本仓库常见的类型：`feat`、`fix`、`refactor`、`docs`、`chore`、`ci`、
`test`。scope 可选（如 `fix(tailwind): ...`、`chore(deps): ...`）。

示例：

```
feat(server): add worker list endpoint
fix(tailwind): scope content sources explicitly
docs: add contribution guide
```

## 报告 Bug

提交 Bug 报告时请尽量包含：

- 你在做什么操作，预期发生什么。
- 实际发生了什么，包括报错信息或日志。
- 你的平台（操作系统、版本）以及运行 DryDrop 的方式。
- 复现步骤（如适用）。

## 许可证

提交贡献即表示你同意这些贡献将基于
[MIT License](../../LICENSE) 授权。
