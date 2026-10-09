# DryDrop

English | [简体中文](docs/zh/README.md)

<p align="center">
  <strong>Let your code run anywhere like water.</strong>
</p>

<p align="center">
  A self-hosted deployment platform for running applications across your own hardware, cloud servers, and serverless environments.
</p>

## What is DryDrop?

DryDrop is an open-source project exploring a simpler, more flexible way to build, deploy, and operate applications across different environments.

Your application should not be tied to a particular cloud provider, a single machine, or a specific deployment workflow.

With DryDrop, the goal is to make deployment feel less like infrastructure management and more like telling your code where to run.

Whether you want to turn an old computer into a personal server, deploy an application to a cloud VPS, or run compatible workloads in a serverless environment, DryDrop aims to provide a unified experience.

## Vision

Infrastructure should adapt to applications, not the other way around.

Today, deploying an application across different environments often means learning different tools, maintaining separate configurations, and building provider-specific workflows. Self-hosting can require considerable operational knowledge, while moving between hosting environments can introduce additional complexity.

DryDrop explores a different approach:

- Run anywhere. Choose the environment that fits your application, hardware, and budget.
- Self-host by default. Make better use of hardware you already own and retain control over your applications.
- Automate everything. Turn repetitive deployment and maintenance tasks into reusable workflows.
- Stay flexible. Keep deployment choices open instead of locking your entire workflow to one provider.
- Observe and operate. Bring deployment status, application health, and operational workflows into one place.

The long-term goal is to make application deployment accessible without hiding the infrastructure that developers need to understand and control.

## Technology

DryDrop is built around a Rust-first development philosophy.

The broader project direction includes:

- Rust for systems components and deployment tooling.
- WebAssembly as a potential portability target for compatible applications.
- Declarative configuration for describing deployment targets and workflows.
- Composable components to keep infrastructure integrations and execution backends extensible.

Specific implementation choices will follow the needs of the MVP rather than requiring every planned component from the beginning.

## Get Started

> DryDrop is in early development — APIs and workflows may change.

### Prerequisites

- [mise](https://mise.jdx.dev) — manages the pinned toolchain (Rust, `dx`, pnpm, ...).
- Docker — required by the server component (PostgreSQL, Redis, RabbitMQ).

### Run from source

```bash
git clone https://github.com/Homeryland/DryDrop.git
cd DryDrop
mise install
mise bootstrap
```

Then pick the app you want to run:

```bash
mise run web-dev       # Web app
mise run macos-dev     # Desktop (macOS; use windows-dev on Windows)
mise run cli-dev       # CLI
mise run tui-dev       # TUI
mise run server-dev    # Server (starts infra via Docker automatically)
```

For per-platform build commands and artifact locations, see
[docs/en/build.md](docs/en/build.md). For the full development guide, see
[docs/en/develop.md](docs/en/develop.md).

## Contributing

Contributions are welcome! DryDrop is open source under the MIT License.

- **Issues** — bug reports and feature ideas are equally appreciated.
- **Pull requests** — keep them focused; for larger changes, open an issue
  first to discuss the approach.
- **Before submitting** — make sure these pass (CI runs the same checks):

  ```bash
  mise run fmt
  mise run check
  mise run clippy
  mise run test
  ```

See [docs/en/develop.md](docs/en/develop.md) for environment setup, project
conventions, and architecture notes.

## License

DryDrop is licensed under the [MIT License](LICENSE).
