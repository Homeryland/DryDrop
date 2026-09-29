# DryDrop

[English](README.md) | [简体中文](docs/zh/README.md)

> **A native developer control center for Cloudflare and your infrastructure.**

DryDrop is a developer infrastructure product by **Homeryland**. It brings local projects, the Cloudflare Developer Platform, development environments, deployments, resource management, logs, and observability into one modern, native workspace.

Available for **CLI**, **TUI**, **Mobile**, **Desktop**, and **Web**.

For more information, visit [drydrop.homeryland.org](https://drydrop.homeryland.org).

## Positioning

DryDrop is not a Cloudflare replacement, a Dashboard clone, or another self-hosted PaaS. It is a **native control center for the Cloudflare Developer Platform**, built on the official ecosystem:

- **Wrangler** handles project workflows such as local development, builds, and deployment.
- **Cloudflare APIs** provide account, resource, domain, management, and observability capabilities.
- **workerd and Miniflare** provide the Worker runtime and local simulation ecosystem.

The initial focus is deep Cloudflare Workers integration. The broader vision is a unified control plane for cloud services and developer-owned infrastructure.

## What You Can Do

- Discover and organize local Worker projects
- Run Workers locally and inspect local or remote bindings
- Deploy across environments with version history and safe rollback
- Manage domains, routes, variables, and secrets
- Work with D1, KV, R2, Queues, Durable Objects, Workflows, Workers AI, and Vectorize
- Explore logs, traces, metrics, and errors per project and deployment

## Principles

- **Local First** — your project is the source of truth; DryDrop uses standard Wrangler configuration.
- **Cloud Native** — a consistent, project-oriented view of the Cloudflare Developer Platform.
- **Native Experience** — built with Rust, GPUI, and gpui-kit; not a WebView wrapper.
- **Security by Design** — OAuth and scoped tokens, OS credential storage, environment isolation, and secret masking.

## Vision

> **Your infrastructure, in one place.**

> **Write code anywhere. Deploy anywhere. Observe everything.**

## License

DryDrop is licensed under the [MIT License](LICENSE).
