# Contributing to DryDrop

Thanks for your interest in contributing! This guide explains how to report
issues, submit changes, and what to check before opening a pull request.

[English](../en/contribution.md) | [简体中文](../zh/contribution.md)

## Ways to contribute

- **Bug reports** — something broken or behaving unexpectedly? Open an issue
  with steps to reproduce, what you expected, and what happened.
- **Feature requests** — describe the problem you are trying to solve, not
  just the solution. This helps us evaluate fit with the project direction.
- **Documentation** — fixes and improvements to docs are always welcome.
- **Code** — bug fixes, refactorings, and new features. See below.

## Getting started

Set up your development environment first (see
[develop.md](../en/develop.md) for the full guide):

```bash
mise install                 # install pinned tools (Rust, dx, pnpm, ...)
mise bootstrap               # rustup targets + Android NDK + pnpm install + Tailwind CSS
```

Then pick the app you want to work on and start it with the matching
`mise run *-dev` task (`web-dev`, `macos-dev`, `ios-dev`, `android-dev`,
`cli-dev`, `tui-dev`, `server-dev`).

## Before you submit

CI runs the same commands locally, so make sure they pass:

```bash
mise run fmt      # cargo fmt --all -- --check
mise run check    # cargo check --workspace --all-features
mise run clippy   # cargo clippy --workspace --all-features -- -D warnings
mise run test     # cargo test --workspace --all-features
```

Additional notes:

- If your changes affect any scanned UI source, regenerate the committed
  Tailwind asset — CI fails if it is stale (see [develop.md](../en/develop.md)).
- Integration tests that touch PostgreSQL / Redis / RabbitMQ need the test
  infrastructure first (`mise run infra-test-up`).
- Please also read the conventions and gotchas section in
  [develop.md](../en/develop.md) — it covers the mistakes most likely to break CI.

## Pull requests

- Keep PRs focused on a single change. For larger features, open an issue
  first so we can discuss the approach before you invest time.
- Rebase onto the latest `main` before submitting.
- Write PR descriptions that explain **why** the change is needed, not just
  what it does.

## Commit messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short summary>
```

Common types seen in this repo: `feat`, `fix`, `refactor`, `docs`, `chore`,
`ci`, `test`. Scope is optional (e.g. `fix(tailwind): ...`,
`chore(deps): ...`).

Examples:

```
feat(server): add worker list endpoint
fix(tailwind): scope content sources explicitly
docs: add contribution guide
```

## Reporting bugs

When opening a bug report, please include:

- What you were doing and what you expected to happen.
- The actual behavior, including error messages or logs.
- Your platform (OS, version) and how you are running DryDrop.
- Steps to reproduce, if applicable.

## License

By contributing, you agree that your contributions will be licensed under the
[MIT License](../../LICENSE).
