# AGENTS.md

Guidance for AI agents and contributors working in the **DryDrop** repository.

## What is DryDrop

DryDrop is a native developer control center for the Cloudflare Developer
Platform and developer-owned infrastructure, built by **Homeryland**. It ships
as CLI, TUI, Desktop, Mobile, and Web apps, all sharing one Rust codebase.
See `README.md` (English) and `docs/zh/README.md` (中文) for product details.

- License: MIT
- Website: https://drydrop.homeryland.org
- Stack: Rust 2024 edition, Dioxus 0.7 (desktop/mobile/web), Axum + Toasty
  (server), Ratatui (TUI), usage-rs (CLI), Tailwind CSS v4 (shared UI).

## Repository layout

```
apps/
  cli/       drydrop CLI (usage-rs), binary name "drydrop"
  desktop/   Dioxus desktop app   (feature: dioxus/desktop)
  mobile/    Dioxus mobile app    (feature: dioxus/mobile; iOS/Android)
  server/    Axum + Toasty(PostgreSQL) + utoipa/OpenAPI backend
  tui/       Ratatui + Crossterm TUI
  web/       Dioxus web app       (feature: dioxus/web)
crates/
  dioxus-components/  Shared Dioxus UI components + Tailwind CSS pipeline
  drydrop/            Facade crate; re-exports drydrop-domain behind feature
                      flags (server/desktop/mobile/web/tui/cli)
  drydrop-domain/     Domain models & schemas (auth, errors); optional toasty
  drydrop-infra/      Infrastructure helpers (currently empty stub)
config/               .env.dev / .env.test / .env.prod templates
deployments/
  docker/             docker compose for postgres, redis, rabbitmq
docs/                 Documentation
.github/workflows/    GitHub Actions CI
```

The workspace is defined in the root `Cargo.toml` (`members = ["crates/*", "apps/*"]`,
`resolver = "3"`, edition 2024).

## Toolchain (mise)

All tool versions are pinned in `mise.toml`:

- `rust` 1.98.1 (with `wasm32-unknown-unknown` and iOS/Android targets),
  managed via `mr-boxington` (`mbx`, a Cargo cache wrapper — `cargo ...` and
  `mbx ...` are interchangeable)
- `cargo:dioxus-cli` 0.7.10 (`dx`)
- `pnpm` 12.5.1
- `hurl`, `watchexec`

## Setup

```bash
mise install                 # install pinned tools
mise run bootstrap           # pnpm install + build Tailwind CSS
```

## Common commands

Use the root `mise run <task>` tasks (defined in `mise.toml`), or the plain
Cargo equivalents:

| Task / command | What it does |
|---|---|
| `mise run check` | `mbx check --workspace --all-features` |
| `mise run test` | `mbx test --workspace --all-features` |
| `mise run clippy` | `mbx clippy --workspace --all-features -- -D warnings` |
| `mise run fmt` | `mbx fmt --all -- --check` |
| `cargo check --workspace --all-features --locked` | Compile everything (CI uses `--locked`) |
| `mise run web` | Dev: Tailwind watch + `dx serve` (web) |
| `mise run desktop` | Dev: Tailwind + desktop app |
| `mise run ios` / `mise run android` | Dev: mobile on iOS / Android |
| `mise run cli` / `mise run tui` | Dev: CLI / TUI |
| `mise run server-dev` | Bring up infra (docker) then run the server |
| `mise run infra-test-up` / `infra-test-down` | Start/stop test infra (postgres, redis, rabbitmq) |

The root tasks are what CI mirrors, so keep them working.

## Tailwind CSS pipeline

- Input: `crates/dioxus-components/tailwind.css`
- Output (committed): `crates/dioxus-components/assets/tailwind.css`
- `@import "tailwindcss" source(none)` is used **on purpose**: automatic source
  detection is disabled and only the explicit `@source` dirs are scanned
  (the component crate itself plus `apps/{web,desktop,mobile}/src`). This keeps
  CI config, docs, and other non-UI files from leaking utility classes into the
  generated CSS.

Regenerate after changing any scanned source or the input CSS:

```bash
pnpm exec tailwindcss -i crates/dioxus-components/tailwind.css -o crates/dioxus-components/assets/tailwind.css
```

The generated asset is committed; CI (`tailwind` job) fails if it is stale.
If you add a class in a file outside the `@source` dirs, add the dir to
`tailwind.css` — do not rely on auto-detection.

## Server architecture

Layered layout inside `apps/server/src`:

- `application/` — `state.rs` (AppState), commands, queries, dtos, error
- `infrastructure/` — config, caches (moka/redis), mail, messaging, persistence
- `presentation/http/v1/` — handlers, routes, middlewares (cors), openapi,
  response, log

Routing (`presentation/http/v1/routes/mod.rs::create_routers`):

- API routes are nested under `/api/v1`; the OpenAPI/Swagger router is merged
  only when `mode == "dev"`.
- `AppState` connects to PostgreSQL via `toasty::Db` and pushes the schema on
  startup, so a running server needs the Postgres instance from `deployments/docker`.

Runtime config:

- `RUST_ENV` selects `config/.env.{dev,test,prod}` (loaded from
  `../../config/.env.<mode>` relative to the server). Not needed for compile.
- Config values can be overridden with env vars using `__` as separator
  (e.g. `POSTGRES__HOST`).

Infra for tests: `mise run infra-test-up` starts postgres/redis/rabbitmq via
docker compose (`deployments/docker/mise.toml`, env files in `config/`).

## Gotchas & conventions

1. **`drydrop` facade crate has no default features.** `drydrop/src/lib.rs`
   always declares `pub mod auth`, which re-exports `drydrop-domain`. You must
   build it with a feature enabled (each app enables its own); CI uses
   `--all-features`.
2. **Use pnpm, not npm.** `pnpm-lock.yaml` + `pnpm-workspace.yaml` are the
   committed lockfiles. `package-lock.json` is stale/out of sync — do not run
   `npm ci`.
3. **`cargo fmt --check` fails on empty `.rs` files.** rustfmt 1.9 normalizes
   a 0-byte module file to a single newline. Keep stub modules non-empty
   (a blank line or a comment is fine).
4. **Shared UI.** `apps/{web,desktop,mobile}` all render the same components
   from `crates/dioxus-components` (`landing::navbar::Navbar`,
   `landing::theme_toggle::ThemeToggle`). A component change affects all three
   platforms at once. Each app's `App` mounts `CSS {}` + `Router::<Route>`.
5. **Errors:** apps use `snafu`; shared libs use `thiserror`
   (`drydrop-domain::errors`).
6. **Serialization:** `serde`/`serde_json`, `jiff` for time (with serde),
   `uuid` v7 (with serde + js).
7. **Desktop on Linux** requires system packages (`libwebkit2gtk-4.1-dev`,
   `libgtk-3-dev`, ...) — CI installs them in `.github/workflows/ci.yml`.
8. **No tests exist yet** (as of Oct 2026); `cargo test --workspace
   --all-features` compiles and runs zero tests. When adding integration tests
   that touch Postgres/Redis/RabbitMQ, start infra first.

## CI

`.github/workflows/ci.yml` runs on every push and pull request:

- `fmt` — `cargo fmt --all -- --check`
- `clippy` — `cargo clippy --workspace --all-features --locked -- -D warnings`
- `test` — `cargo test --workspace --all-features --locked`
- `tailwind` — rebuilds `assets/tailwind.css` and asserts it is committed
  (`git diff --exit-code`)

Runs on `ubuntu-24.04` with the pinned Rust toolchain `1.98.1` (keep in sync
with `mise.toml`).

## Adding a feature

- **Server endpoint:** add a handler under `presentation/http/v1/handlers/...`,
  a route in `presentation/http/v1/routes/`, register it in
  `create_routers()`, add DTOs, and document it with `utoipa` in `openapi.rs`.
- **UI component:** add under `crates/dioxus-components/src`, export it from
  `crates/dioxus-components/src/lib.rs`, use Tailwind classes, and regenerate
  the CSS asset.
- **New app/crate:** add to the workspace (root `Cargo.toml` `members`),
  wire platform features through the `drydrop` facade, and add a `mise.toml`
  task if it has a dev command.