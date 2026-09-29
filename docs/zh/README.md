# DryDrop

[English](../../README.md) | [简体中文](README.md)

> **面向 Cloudflare 与开发者基础设施的原生控制中心。**

DryDrop 是 **Homeryland** 旗下的开发者基础设施产品，将本地项目、Cloudflare Developer Platform、开发环境、部署、资源管理、日志与可观测性整合到一个现代化的原生工作台中。

提供 **CLI**、**TUI**、**Mobile**、**Desktop** 与 **Web** 版本。

了解更多请访问 [drydrop.homeryland.org](https://drydrop.homeryland.org)。

## 定位

DryDrop 不是 Cloudflare 的替代品，不是 Dashboard 的复刻，也不是另一个 Self-hosted PaaS。它是 **Cloudflare Developer Platform 的原生控制中心**，建立在官方生态之上：

- **Wrangler** 负责本地开发、构建、部署等项目工作流。
- **Cloudflare APIs** 提供账户、资源、域名、管理与可观测性能力。
- **workerd 与 Miniflare** 提供 Worker Runtime 与本地模拟生态。

当前首先专注于深度集成 Cloudflare Workers；更长远的愿景，是成为连接云服务与开发者自有基础设施的统一控制平面。

## 核心能力

- 发现和组织本地 Worker 项目
- 在本地运行 Worker，查看本地或远程 Bindings
- 跨环境部署，查看版本历史并安全回滚
- 管理域名、路由、变量和 Secrets
- 使用 D1、KV、R2、Queues、Durable Objects、Workflows、Workers AI 与 Vectorize
- 按项目与部署探索日志、Trace、Metrics 与错误

## 核心理念

- **Local First** — 项目是事实来源，直接使用标准 Wrangler 配置。
- **Cloud Native** — 以一致、以项目为中心的界面呈现 Cloudflare Developer Platform。
- **Native Experience** — 采用 Rust、GPUI 与 gpui-kit 构建，而非 WebView 包装器。
- **安全设计** — OAuth 与限定权限 Token、操作系统凭据存储、环境隔离与 Secret Masking。

## 愿景

> **你的基础设施，尽在一处。**

> **Write code anywhere. Deploy anywhere. Observe everything.**

## 许可证

DryDrop 基于 [MIT License](LICENSE) 开源。
