# Developing DryDrop

This guide covers day-to-day development: environment setup, running each app
locally, checks and tests, and the conventions the codebase follows.

For build commands and artifact locations, see [build.md](../en/build.md).

[English](../en/develop.md) | [简体中文](../zh/develop.md)

## Environment setup

```bash
mise install                 # install pinned tools (Rust, dx, pnpm, hurl, watchexec, ...)
mise bootstrap               # rustup targets + Android NDK + pnpm install + Tailwind CSS
```

Extra platform notes:

- **Desktop on Linux** requires system packages (`libwebkit2gtk-4.1-dev`,
  `libgtk-3-dev`, ...).
- **Android** needs the NDK — `mise bootstrap` downloads it automatically.
- **Server (and its tests)** need Docker for PostgreSQL / Redis / RabbitMQ.

## Project layout

```
apps/
  cli/        drydrop CLI (usage-rs), binary name "drydrop"
  desktop/    Dioxus desktop app
  mobile/     Dioxus mobile app (iOS/Android)
  server/     Axum + Toasty (PostgreSQL) + utoipa/OpenAPI backend
  tui/        Ratatui + Crossterm TUI
  web/        Dioxus web app
crates/
  dioxus-components/   Shared Dioxus UI components + Tailwind CSS pipeline
  drydrop/             Facade crate; re-exports drydrop-domain behind feature flags
  drydrop-domain/      Domain models & schemas (auth, errors); optional toasty
  drydrop-infra/       Infrastructure helpers (currently empty stub)
```

The workspace is defined in the root `Cargo.toml`; all tool versions are pinned
in `mise.toml`.

## Running apps locally

All commands run from the repository root. Dev tasks for the UI apps also
rebuild Tailwind CSS first.

| App             | Command                  | What it does |
|-----------------|--------------------------|--------------|
| Web             | `mise run web-dev`       | Tailwind watch + `dx serve` (web) |
| Desktop (macOS) | `mise run macos-dev`     | Tailwind + `dx serve --macos` |
| Desktop (Windows) | `mise run windows-dev` | Run on a Windows machine |
| iOS             | `mise run ios-dev`       | Tailwind + `dx serve --ios` |
| Android         | `mise run android-dev`   | Tailwind + `dx serve --android` |
| CLI             | `mise run cli-dev`       | `watchexec` rebuild on change |
| TUI             | `mise run tui-dev`       | `watchexec` rebuild on change |
| Server          | `mise run server-dev`    | Starts infra (docker), runs the server, stops infra afterwards |

### Server configuration

- `RUST_ENV` selects `config/.env.{dev,test,prod}` (loaded relative to the
  server). Not needed for compiling.
- Config values can be overridden with env vars using `__` as separator, e.g.
  `POSTGRES__HOST`.
- `AppState` pushes the schema to PostgreSQL on startup, so a running server
  needs the Postgres instance from `deployments/docker` (started automatically
  by `mise run server-dev`, or manually via `mise run infra-dev-up`).

## Checks and tests

These are the commands CI mirrors — keep them green:

```bash
mise run check    # mbx check --workspace --all-features
mise run test     # mbx test --workspace --all-features
mise run clippy   # mbx clippy --workspace --all-features -- -D warnings
mise run fmt      # mbx fmt --all -- --check
```

Integration tests that touch PostgreSQL / Redis / RabbitMQ need the test
infrastructure first:

```bash
mise run infra-test-up      # start postgres/redis/rabbitmq (docker compose)
# ... run tests ...
mise run infra-test-down    # stop it again
```

## Tailwind CSS pipeline

- Input: `crates/dioxus-components/tailwind.css`
- Output (committed): `crates/dioxus-components/assets/tailwind.css`

Regenerate after changing any scanned source or the input CSS:

```bash
pnpm exec tailwindcss -i crates/dioxus-components/tailwind.css -o crates/dioxus-components/assets/tailwind.css
```

Notes:

- `@import "tailwindcss" source(none)` disables automatic source detection on
  purpose — only the explicit `@source` dirs are scanned (the component crate
  itself plus `apps/{web,desktop,mobile}/src`).
- The generated asset is committed; CI fails if it is stale.
- If you add a class in a file outside the `@source` dirs, add the dir to
  `tailwind.css` — do not rely on auto-detection.

## Conventions & gotchas

1. **`drydrop` facade crate has no default features.** It always declares
   `pub mod auth` (re-exporting `drydrop-domain`), so you must build it with a
   feature enabled — each app enables its own; CI uses `--all-features`.
2. **Use pnpm, not npm.** `pnpm-lock.yaml` + `pnpm-workspace.yaml` are the
   committed lockfiles. `package-lock.json` is stale — do not run `npm ci`.
3. **`cargo fmt --check` fails on empty `.rs` files.** rustfmt normalizes a
   0-byte module file to a single newline; keep stub modules non-empty.
4. **Shared UI.** `apps/{web,desktop,mobile}` render the same components from
   `crates/dioxus-components`; a component change affects all three platforms
   at once. Each app's `App` mounts `CSS {}` + `Router::<Route>`.
5. **Errors:** apps use `snafu`; shared libs use `thiserror`
   (`drydrop-domain::errors`).
6. **Serialization:** `serde`/`serde_json`, `jiff` for time (with serde),
   `uuid` v7 (with serde + js).
7. **Platform features live on the dependency lines.** The UI apps do not
   declare their own Cargo `[features]`; each enables its platform directly
   (`dioxus = { workspace = true, features = ["desktop"] }` and
   `drydrop = { features = ["desktop"] }`). The shared `dioxus` workspace
   dependency enables `router` only — don't re-add `fullstack` or app-local
   feature tables.

## Adding a feature

- **Server endpoint:** add a handler under
  `apps/server/src/presentation/http/v1/handlers/...`, a route in
  `presentation/http/v1/routes/`, register it in `create_routers()`, add DTOs,
  and document it with `utoipa` in `openapi.rs`.
- **UI component:** add under `crates/dioxus-components/src`, export it from
  `crates/dioxus-components/src/lib.rs`, use Tailwind classes, and regenerate
  the CSS asset.
- **New app/crate:** add to the workspace (root `Cargo.toml` `members`), wire
  platform features through the `drydrop` facade, and add a `mise.toml` task
  if it has a dev command.
