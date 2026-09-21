# DryDrop

[English](README.md) | [简体中文](README_zh-CN.md)

> **A native developer control center for Cloudflare and your infrastructure.**

DryDrop is a developer infrastructure product by **Homeryland**. It brings local projects, the Cloudflare Developer Platform, development environments, deployments, resource management, logs, and observability into one modern, native workspace.

DryDrop does not attempt to replace Cloudflare, Wrangler, or the official Workers runtime. It builds on the Cloudflare ecosystem and turns its capabilities into a cohesive, high-performance developer experience.

> **Make cloud infrastructure feel local.**

## Why DryDrop?

Working with Cloudflare Workers often means moving repeatedly between an editor, terminal, Wrangler, the Cloudflare Dashboard, logs, and analytics. DryDrop is designed to connect that fragmented workflow:

```text
                     DryDrop
                        │
        ┌───────────────┼───────────────┐
        │               │               │
     Develop          Deploy          Observe
        │               │               │
        └───────────────┼───────────────┘
                        │
                  Cloudflare
```

From a local Worker project, developers can manage the complete application lifecycle in one place:

- Discover and organize local projects
- Run Workers locally
- Inspect and manage bindings
- Deploy to multiple environments
- Review versions and deployment history
- Roll back safely
- Explore logs, traces, metrics, and errors
- Manage domains, routes, variables, and secrets
- Work with D1, KV, R2, Queues, Durable Objects, Workflows, Workers AI, and Vectorize

## Product Positioning

DryDrop is not a Cloudflare replacement, a Dashboard clone, or another self-hosted PaaS such as Coolify or Dokploy.

It is a **native control center for the Cloudflare Developer Platform**, combining:

```text
Cloudflare Dashboard
        +
Wrangler
        +
Local Development
        +
Git
        +
Observability
        +
Native Desktop UX
```

The initial focus is deep Cloudflare Workers integration. The broader vision is a unified developer control plane for cloud services and developer-owned infrastructure.

## Core Principles

### Local First

Your project remains the source of truth. DryDrop discovers existing Worker projects and understands standard files and directories such as:

```text
wrangler.jsonc
wrangler.toml
package.json
src/
.git/
```

DryDrop uses Wrangler configuration instead of introducing another proprietary Worker configuration format.

### Cloud Native

DryDrop understands the Cloudflare Developer Platform and presents its resources through a consistent project-oriented interface:

- Workers
- D1
- KV
- R2
- Queues
- Durable Objects
- Workflows
- Workers AI
- Vectorize
- Domains and routes
- Variables and secrets

### Native Experience

DryDrop Desktop is designed as a native application rather than a WebView wrapper. Its desktop stack is built with **Rust**, **GPUI**, and **gpui-kit**, with a keyboard-first experience inspired by tools such as Zed, Docker Desktop, OrbStack, and TablePlus.

The native workspace is designed around command palettes, split panes, project trees, terminals, log viewers, resource inspectors, deployment timelines, real-time status, native notifications, and multi-window workflows.

## The DryDrop Experience

### Project Manager

DryDrop discovers local Worker projects and brings together their local and remote context:

```text
Project
├── Path and runtime
├── Framework
├── Git branch and commit
├── Environment
├── Cloudflare account
└── Current deployment
```

### Local Development and Explorer

DryDrop integrates Wrangler's local development workflow and the Cloudflare runtime ecosystem based on `workerd` and Miniflare. Local and remote bindings can be surfaced directly alongside the running Worker.

```text
Project
├── Worker
├── D1
│   ├── Tables
│   └── SQL Console
├── KV
│   └── Keys
├── R2
│   └── Objects
└── Logs and traces
```

The goal is simple: inspecting a KV value or running a D1 query should not require leaving the workspace.

### Deployment Center

Deployment is a first-class workflow that connects source control, environments, Cloudflare versions, and operational context:

```text
Validate configuration
        ↓
Check Git status
        ↓
Build and upload
        ↓
Create deployment
        ↓
Verify and observe
```

Each deployment can be associated with its branch, commit, author, environment, duration, status, logs, and resources. Version history and rollback are presented as primary operations rather than hidden behind commands or API calls.

### Resource Explorer and Worker Inspector

DryDrop provides a unified view of Cloudflare resources and a dedicated inspector for every Worker:

```text
Worker
├── Overview
├── Deployments and versions
├── Domains and routes
├── Bindings
├── Variables and secrets
├── Logs, metrics, and traces
└── Settings
```

Resource pages connect configuration, usage, bindings, related Workers, and operational data without requiring developers to memorize every Wrangler command.

### Environments and Git

DryDrop models the path from code to production explicitly:

```text
Git repository
      ↓
DryDrop project
      ↓
Environment
      ↓
Deployment
      ↓
Cloudflare version
      ↓
Observability
```

Development, preview, staging, and production environments can each carry their own bindings, variables, secrets, domains, resources, and deployment policies.

### Observability

DryDrop connects production behavior to the exact version that introduced it. Logs, requests, errors, latency, metrics, and traces are organized around projects and deployments so developers can answer not only whether a Worker is running, but what changed after a release.

## Built on the Cloudflare Ecosystem

DryDrop relies on official Cloudflare capabilities rather than reimplementing them:

- **Wrangler** handles project workflows such as local development, builds, and deployment.
- **Cloudflare APIs** provide account, resource, domain, metadata, management, and observability capabilities.
- **workerd and Miniflare** provide the Worker runtime and local simulation ecosystem.

```text
DryDrop
├── Project and deployment experience
├── Wrangler integration
└── Cloudflare API integration
           ↓
       Cloudflare
```

This approach keeps DryDrop compatible with the tools Cloudflare developers already use while providing a more integrated native workflow.

## Security by Design

Infrastructure credentials and secrets require first-class protection. DryDrop is designed around:

- OAuth and scoped API tokens
- Least-privilege access
- Operating-system credential storage
- Environment isolation
- Secret masking
- Auditability

DryDrop does not use the Cloudflare Global API Key as the default authentication approach and never exposes secret values by default.

## Who Is DryDrop For?

DryDrop is built for Cloudflare Workers developers, including:

- JavaScript, TypeScript, and Rust developers
- Independent developers and indie hackers
- Full-stack developers and small teams
- Startups building on Cloudflare Workers
- Homelab and self-hosted infrastructure enthusiasts

## Long-Term Vision

DryDrop begins with Cloudflare, but its identity is broader than a “Cloudflare GUI.” The long-term goal is to give developers one control plane for Cloudflare, local machines, edge devices, NAS systems, and remote servers—without forcing them to constantly switch between dashboards, terminals, SSH sessions, and monitoring tools.

> **Your infrastructure, in one place.**

> **Write code anywhere. Deploy anywhere. Observe everything.**

## License

DryDrop is licensed under the [MIT License](LICENSE).
