# DryDrop

[English](README.md) | [简体中文](README_zh-CN.md)

> **面向 Cloudflare 与开发者基础设施的原生控制中心。**

DryDrop 是 **Homeryland** 旗下的开发者基础设施产品，将本地项目、Cloudflare Developer Platform、开发环境、部署、资源管理、日志与可观测性整合到一个现代化的原生工作台中。

DryDrop 不试图替代 Cloudflare、Wrangler 或官方 Workers Runtime，而是建立在 Cloudflare 官方生态之上，将已有能力组织成统一、流畅且高性能的开发者体验。

> **让云基础设施拥有本地开发般的体验。**

## 为什么选择 DryDrop？

开发 Cloudflare Workers 时，开发者通常需要不断往返于代码编辑器、终端、Wrangler、Cloudflare Dashboard、日志和分析工具之间。DryDrop 希望将这些分散的工作流连接起来：

```text
                     DryDrop
                        │
        ┌───────────────┼───────────────┐
        │               │               │
       开发             部署             观测
        │               │               │
        └───────────────┼───────────────┘
                        │
                  Cloudflare
```

开发者可以从一个本地 Worker 项目出发，在同一处管理完整的应用生命周期：

- 发现和组织本地项目
- 在本地运行 Worker
- 查看和管理 Bindings
- 部署到不同环境
- 查看版本与部署历史
- 安全地执行回滚
- 探索日志、Trace、Metrics 与错误
- 管理域名、路由、变量和 Secrets
- 使用 D1、KV、R2、Queues、Durable Objects、Workflows、Workers AI 与 Vectorize

## 产品定位

DryDrop 不是 Cloudflare 的替代品，不是 Dashboard 的复刻，也不是另一个 Coolify 或 Dokploy 式的 Self-hosted PaaS。

DryDrop 是 **Cloudflare Developer Platform 的原生控制中心**，将以下能力整合到一起：

```text
Cloudflare Dashboard
        +
Wrangler
        +
本地开发
        +
Git
        +
可观测性
        +
原生桌面体验
```

DryDrop 当前首先专注于深度集成 Cloudflare Workers；更长远的愿景，是成为连接云服务与开发者自有基础设施的统一控制平面。

## 核心理念

### Local First

项目始终属于开发者，并作为配置的事实来源。DryDrop 会发现现有 Worker 项目，并理解其中的标准文件与目录：

```text
wrangler.jsonc
wrangler.toml
package.json
src/
.git/
```

DryDrop 直接使用 Wrangler 配置，不会再创造一套私有的 Worker 配置格式。

### Cloud Native

DryDrop 原生理解 Cloudflare Developer Platform，并通过一致、以项目为中心的界面呈现其资源：

- Workers
- D1
- KV
- R2
- Queues
- Durable Objects
- Workflows
- Workers AI
- Vectorize
- Domains 与 Routes
- Variables 与 Secrets

### Native Experience

DryDrop Desktop 被设计为真正的原生应用，而不是 WebView 包装器。桌面端采用 **Rust**、**GPUI** 和 **gpui-kit**，提供以键盘操作为核心的使用体验，其设计目标接近 Zed、Docker Desktop、OrbStack 与 TablePlus 等现代开发者工具。

原生工作台围绕 Command Palette、分栏、项目树、终端、日志查看器、资源检查器、部署时间线、实时状态、原生通知与多窗口工作流构建。

## DryDrop 核心体验

### 项目管理

DryDrop 自动发现本地 Worker 项目，并将本地信息与远程状态整合到一起：

```text
项目
├── 路径与 Runtime
├── Framework
├── Git Branch 与 Commit
├── Environment
├── Cloudflare Account
└── 当前 Deployment
```

### 本地开发与 Local Explorer

DryDrop 集成 Wrangler 的本地开发工作流，以及基于 `workerd` 和 Miniflare 的 Cloudflare Runtime 生态。本地模拟资源与 Remote Bindings 可以和正在运行的 Worker 一同呈现。

```text
项目
├── Worker
├── D1
│   ├── Tables
│   └── SQL Console
├── KV
│   └── Keys
├── R2
│   └── Objects
└── Logs 与 Traces
```

目标很简单：开发者不应该因为查看一条 KV 数据或执行一条 D1 SQL 而离开当前工作台。

### Deployment Center

部署是 DryDrop 的一级工作流，它将源代码、环境、Cloudflare Version 与运行状态连接起来：

```text
验证配置
   ↓
检查 Git 状态
   ↓
构建并上传
   ↓
创建 Deployment
   ↓
验证并观测
```

每次部署都可以关联 Branch、Commit、Author、Environment、Duration、Status、Logs 与 Resources。版本历史和 Rollback 是直接可用的核心操作，而不是隐藏在命令或 API 之后。

### Resource Explorer 与 Worker Inspector

DryDrop 为 Cloudflare 资源提供统一视图，并为每个 Worker 提供专属 Inspector：

```text
Worker
├── Overview
├── Deployments 与 Versions
├── Domains 与 Routes
├── Bindings
├── Variables 与 Secrets
├── Logs、Metrics 与 Traces
└── Settings
```

资源页面将配置、用量、Bindings、关联 Workers 与运行数据连接起来，让开发者无需记住每一条 Wrangler 命令。

### Environments 与 Git

DryDrop 对从代码到生产环境的完整链路进行显式建模：

```text
Git Repository
      ↓
DryDrop Project
      ↓
Environment
      ↓
Deployment
      ↓
Cloudflare Version
      ↓
Observability
```

Development、Preview、Staging 与 Production 环境可以分别拥有自己的 Bindings、Variables、Secrets、Domains、Resources 与部署策略。

### 可观测性

DryDrop 将生产环境中的实际表现关联到引入变化的具体版本。Logs、Requests、Errors、Latency、Metrics 与 Traces 按项目和部署进行组织，帮助开发者了解的不只是“Worker 是否正在运行”，还包括“新版本上线后发生了什么变化”。

## 基于 Cloudflare 官方生态

DryDrop 使用 Cloudflare 的官方能力，而不是重复实现它们：

- **Wrangler** 负责本地开发、构建、部署等项目工作流。
- **Cloudflare APIs** 提供账户、资源、域名、元数据、管理与可观测性能力。
- **workerd 与 Miniflare** 提供 Worker Runtime 与本地模拟生态。

```text
DryDrop
├── 项目与部署体验
├── Wrangler Integration
└── Cloudflare API Integration
           ↓
       Cloudflare
```

这种方式既能保持与 Cloudflare 开发者现有工具链的兼容，也能提供更统一的原生工作流。

## 安全设计

基础设施凭据与 Secrets 必须受到一级保护。DryDrop 围绕以下原则进行设计：

- OAuth 与具有限定权限的 API Token
- 最小权限访问
- 操作系统安全凭据存储
- 环境隔离
- Secret Masking
- 可审计性

DryDrop 不会将 Cloudflare Global API Key 作为默认认证方案，也不会默认显示 Secret 明文。

## 适合谁？

DryDrop 面向 Cloudflare Workers 开发者，尤其适合：

- JavaScript、TypeScript 与 Rust 开发者
- 独立开发者与 Indie Hacker
- 全栈开发者与小型团队
- 使用 Cloudflare Workers 的创业公司
- Homelab 与 Self-hosted 基础设施爱好者

## 长期愿景

DryDrop 从 Cloudflare 出发，但不会将自身限制为“Cloudflare GUI”。长期目标是为开发者提供一个统一的控制平面，连接 Cloudflare、本地计算机、边缘设备、NAS 与远程服务器，减少在 Dashboard、终端、SSH 和监控工具之间频繁切换带来的摩擦。

> **你的基础设施，尽在一处。**

> **Write code anywhere. Deploy anywhere. Observe everything.**

## 许可证

DryDrop 基于 [MIT License](LICENSE) 开源。
