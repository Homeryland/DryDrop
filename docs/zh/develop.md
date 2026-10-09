# 开发 DryDrop

本指南介绍日常开发流程：环境搭建、本地运行各个应用、检查与测试，以及本仓库
遵循的约定。

构建命令与产物位置见 [build.md](../zh/build.md)。

[English](../en/develop.md) | 简体中文

## 环境搭建

```bash
mise install                 # 安装固定版本的工具链（Rust、dx、pnpm、hurl、watchexec 等）
mise bootstrap               # rustup 目标 + Android NDK + pnpm install + Tailwind CSS
```

各平台补充说明：

- **Linux 桌面端**需要系统包（`libwebkit2gtk-4.1-dev`、`libgtk-3-dev` 等）。
- **Android** 需要 NDK —— `mise bootstrap` 会自动下载。
- **服务端（及其测试）**需要 Docker 来运行 PostgreSQL / Redis / RabbitMQ。

## 项目结构

```
apps/
  cli/        drydrop CLI（usage-rs），二进制名 "drydrop"
  desktop/    Dioxus 桌面应用
  mobile/     Dioxus 移动应用（iOS/Android）
  server/     Axum + Toasty（PostgreSQL）+ utoipa/OpenAPI 后端
  tui/        Ratatui + Crossterm TUI
  web/        Dioxus Web 应用
crates/
  dioxus-components/   共享 Dioxus UI 组件 + Tailwind CSS 流水线
  drydrop/             门面 crate；通过 feature flag 转发 drydrop-domain
  drydrop-domain/      领域模型与 schema（auth、errors）；可选 toasty
  drydrop-infra/       基础设施辅助（目前是空的桩模块）
```

Workspace 定义在根目录 `Cargo.toml`；所有工具版本固定在 `mise.toml`。

## 本地运行

以下命令均在仓库根目录执行。UI 应用的 dev 任务会先重新构建 Tailwind CSS。

| 应用            | 命令                     | 说明 |
|-----------------|--------------------------|------|
| Web             | `mise run web-dev`       | Tailwind watch + `dx serve`（web） |
| 桌面（macOS）   | `mise run macos-dev`     | Tailwind + `dx serve --macos` |
| 桌面（Windows） | `mise run windows-dev`   | 需在 Windows 机器上运行 |
| iOS             | `mise run ios-dev`       | Tailwind + `dx serve --ios` |
| Android         | `mise run android-dev`   | Tailwind + `dx serve --android` |
| CLI             | `mise run cli-dev`       | `watchexec` 监听变更并重新构建 |
| TUI             | `mise run tui-dev`       | `watchexec` 监听变更并重新构建 |
| 服务端          | `mise run server-dev`    | 先启动基础设施（docker），运行服务端，结束后自动停止 |

### 服务端配置

- `RUST_ENV` 选择 `config/.env.{dev,test,prod}`（相对服务端目录加载）。编译时
  无需设置。
- 配置项可通过环境变量覆盖，使用 `__` 作为分隔符，例如 `POSTGRES__HOST`。
- `AppState` 会在启动时把 schema 推送到 PostgreSQL，因此运行服务端需要
  `deployments/docker` 中的 Postgres 实例（`mise run server-dev` 会自动启动，
  也可通过 `mise run infra-dev-up` 手动启动）。

## 检查与测试

以下命令与 CI 保持一致 —— 请保持它们全部通过：

```bash
mise run check    # mbx check --workspace --all-features
mise run test     # mbx test --workspace --all-features
mise run clippy   # mbx clippy --workspace --all-features -- -D warnings
mise run fmt      # mbx fmt --all -- --check
```

涉及 PostgreSQL / Redis / RabbitMQ 的集成测试需要先启动测试基础设施：

```bash
mise run infra-test-up      # 启动 postgres/redis/rabbitmq（docker compose）
# ... 运行测试 ...
mise run infra-test-down    # 再停止
```

## Tailwind CSS 流水线

- 输入：`crates/dioxus-components/tailwind.css`
- 输出（已提交）：`crates/dioxus-components/assets/tailwind.css`

修改任何被扫描的源码或输入 CSS 后重新生成：

```bash
pnpm exec tailwindcss -i crates/dioxus-components/tailwind.css -o crates/dioxus-components/assets/tailwind.css
```

说明：

- `@import "tailwindcss" source(none)` 是**有意**禁用自动源码检测的 —— 只扫描
  显式的 `@source` 目录（组件 crate 本身以及 `apps/{web,desktop,mobile}/src`）。
- 生成的资产会提交到仓库；如果过期，CI 会失败。
- 如果在被扫描目录之外的文件里用了新的 class，请把该目录加进
  `tailwind.css` —— 不要依赖自动检测。

## 约定与注意事项

1. **`drydrop` 门面 crate 没有默认 feature。** 它始终声明 `pub mod auth`
   （转发 `drydrop-domain`），因此必须启用某个 feature 才能构建 —— 每个应用
   启用自己的 feature；CI 使用 `--all-features`。
2. **使用 pnpm，不要用 npm。** 仓库提交的锁文件是 `pnpm-lock.yaml` +
   `pnpm-workspace.yaml`。`package-lock.json` 已经过期 —— 不要运行 `npm ci`。
3. **`cargo fmt --check` 对空 `.rs` 文件会失败。** rustfmt 会把 0 字节的模块
   文件规范成单个换行；保持桩模块非空。
4. **共享 UI。** `apps/{web,desktop,mobile}` 渲染同一套
   `crates/dioxus-components` 组件；组件改动会同时影响三个平台。每个应用的
   `App` 都挂载 `CSS {}` + `Router::<Route>`。
5. **错误处理：** 应用层使用 `snafu`；共享库使用 `thiserror`
   （`drydrop-domain::errors`）。
6. **序列化：** `serde`/`serde_json`，时间用 `jiff`（含 serde），ID 用
   `uuid` v7（含 serde + js）。
7. **平台 feature 写在依赖行上。** UI 应用不声明自己的 Cargo `[features]`；
   每个应用直接在依赖行启用平台（`dioxus = { workspace = true, features =
   ["desktop"] }` 和 `drydrop = { features = ["desktop"] }`）。共享的
   `dioxus` workspace 依赖只启用 `router` —— 不要重新添加 `fullstack` 或
   应用本地的 feature 表。

## 新增功能

- **服务端接口：** 在 `apps/server/src/presentation/http/v1/handlers/...`
  下添加 handler，在 `presentation/http/v1/routes/` 中加路由，在
  `create_routers()` 中注册，添加 DTO，并在 `openapi.rs` 中用 `utoipa` 写文档。
- **UI 组件：** 在 `crates/dioxus-components/src` 下添加，从
  `crates/dioxus-components/src/lib.rs` 导出，使用 Tailwind class，并重新生成
  CSS 资产。
- **新应用/crate：** 加入 workspace（根目录 `Cargo.toml` 的 `members`），通过
  `drydrop` 门面接线平台 feature，如有 dev 命令则添加 `mise.toml` 任务。
