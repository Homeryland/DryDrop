# DryDrop

<p align="center">
  <a href="../../README.md">English</a> | 简体中文
</p>

<p align="center">
  <strong>让你的代码像水一样，随处流动运行。</strong>
</p>

<p align="center">
  一个自托管的部署平台，让你在自己的硬件、云服务器和无服务器环境中运行应用。
</p>

## 什么是 DryDrop？

DryDrop 是一个开源项目，探索一种更简单、更灵活的方式，在不同的环境中构建、部署和运维应用。

你的应用不应该被绑定在某一个云服务商、某一台机器或某一种部署流程上。

DryDrop 的目标是让部署不再像基础设施管理，而更像是告诉你的代码："去那里运行"。

无论你想把一台旧电脑变成个人服务器、把应用部署到云 VPS，还是在无服务器环境中运行兼容的工作负载，DryDrop 都致力于提供统一的体验。

## 愿景

基础设施应该适应应用，而不是反过来。

今天，把一个应用部署到不同的环境，往往意味着学习不同的工具、维护多套配置、搭建针对特定服务商的工作流。自托管可能需要相当多的运维知识，而在不同托管环境之间迁移又会带来额外的复杂度。

DryDrop 探索另一种方式：

- **随处运行**。选择适合你的应用、硬件和预算的环境。
- **默认自托管**。更好地利用你已有的硬件，并保留对应用的控制权。
- **一切自动化**。把重复的部署与维护任务变成可复用的工作流。
- **保持灵活**。让部署选择保持开放，而不是把整个工作流锁死在一家服务商上。
- **观察与运维**。把部署状态、应用健康和运维工作流汇聚到一处。

长期目标是让应用部署变得触手可及，同时不隐藏开发者需要理解和掌控的基础设施。

## 技术栈

DryDrop 围绕 Rust 优先的开发理念构建。

更宏观的项目方向包括：

- 使用 Rust 构建系统组件与部署工具。
- 以 WebAssembly 作为兼容应用潜在的可移植性目标。
- 使用声明式配置描述部署目标与工作流。
- 使用可组合的组件，保持基础设施集成与执行后端的可扩展性。

具体的实现选择会跟随 MVP 的实际需求，而不要求一开始就实现所有规划中的组件。

## 快速开始

> DryDrop 处于早期开发阶段 —— API 与工作流可能发生变化。

### 前置要求

- [mise](https://mise.jdx.dev) —— 管理固定版本的工具链（Rust、`dx`、pnpm 等）。
- Docker —— 服务端组件需要（PostgreSQL、Redis、RabbitMQ）。

### 从源码运行

```bash
git clone https://github.com/Homeryland/DryDrop.git
cd DryDrop
mise install
mise bootstrap
```

然后选择你想运行的应用：

```bash
mise run web-dev       # Web 应用
mise run macos-dev     # 桌面端（macOS；Windows 使用 windows-dev）
mise run cli-dev       # CLI
mise run tui-dev       # TUI
mise run server-dev    # 服务端（自动通过 Docker 启动基础设施）
```

各平台的构建命令与产物位置见 [build.md](../zh/build.md)。
完整开发指南见 [develop.md](../zh/develop.md)。

## 贡献

欢迎参与贡献！DryDrop 基于 MIT 许可证开源。

- **Issue** —— Bug 报告与功能建议同样欢迎。
- **Pull Request** —— 请保持改动聚焦；较大的改动建议先开 issue 讨论方案。
- **提交前** —— 确保以下检查通过（CI 运行同样的检查）：

  ```bash
  mise run fmt
  mise run check
  mise run clippy
  mise run test
  ```

环境搭建、项目约定与架构说明见 [develop.md](../zh/develop.md)。

## 许可证

DryDrop 基于 [MIT License](LICENSE) 开源。
