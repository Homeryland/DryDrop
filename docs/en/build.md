# Building DryDrop

This document describes how to build DryDrop for each platform and where the
build artifacts are placed.

## Prerequisites

```bash
mise install                 # install pinned tools (Rust, dx, pnpm, ...)
mise bootstrap               # rustup targets + Android NDK + pnpm install + Tailwind CSS
```

> The Tailwind CSS asset (`crates/dioxus-components/assets/tailwind.css`) must
> exist before building any UI app. `mise bootstrap` generates it.

## Desktop

### macOS

From the repository root:

```bash
mise run macos-build
```

This runs `dx bundle --macos --package-types "macos" --package-types "dmg"` in
`apps/desktop`.

Artifacts:

```
target/dx/drydrop-desktop/bundle/macos/macos/
├── DryDrop Desktop.app
└── DryDrop Desktop_<version>_aarch64.dmg
```

### Windows

TODO

## Mobile

> Note: the current `dx bundle` tasks build **debug** artifacts (no
> `--release` flag).

### iOS

From the repository root:

```bash
mise run ios-build
```

This runs `dx bundle --ios --package-types "ios"` in `apps/mobile` and renames
the app bundle to `DryDrop Mobile.app`.

Artifacts:

```
target/dx/drydrop-mobile/debug/ios/DryDrop Mobile.app
```

### Android

From the repository root:

```bash
mise run android-build
```

This runs `dx bundle --android --package-types "apk"` in `apps/mobile` and
renames the APK to `DryDrop Mobile.apk`.

Artifacts:

```
target/dx/drydrop-mobile/debug/android/app/app/build/outputs/apk/debug/DryDrop Mobile.apk
```

## Web

From the repository root:

```bash
mise run web-build
```

This runs `dx bundle --web` in `apps/web`.

Artifacts (static site — deploy the `public` directory):

```
target/dx/drydrop-web/debug/web/public/
```

(Add `--release` to get an optimized build in
`target/dx/drydrop-web/release/web/public/`.)

## CLI, TUI, Server

From the repository root:

```bash
mise run cli-build         # CLI  (binary name: drydrop)
mise run tui-build         # TUI  (binary name: drydrop-tui)
mise run server-build      # Server (binary name: drydrop-server)
```

Each of these runs `mbx build --release` in the corresponding app directory
(`mbx` is interchangeable with `cargo`, see `mise.toml`).

Artifacts:

| App    | Binary                       |
|--------|------------------------------|
| CLI    | `target/release/drydrop`     |
| TUI    | `target/release/drydrop-tui` |
| Server | `target/release/drydrop-server` |

## Artifact location summary

| Platform      | Build command             | Artifact location                                                                                       |
|---------------|---------------------------|---------------------------------------------------------------------------------------------------------|
| Desktop macOS | `mise run macos-build`    | `target/dx/drydrop-desktop/bundle/macos/macos/`                                                          |
| Desktop Windows | — (TODO)                | —                                                                                                        |
| iOS           | `mise run ios-build`      | `target/dx/drydrop-mobile/debug/ios/DryDrop Mobile.app`                                                  |
| Android       | `mise run android-build`  | `target/dx/drydrop-mobile/debug/android/app/app/build/outputs/apk/debug/DryDrop Mobile.apk`              |
| Web           | `mise run web-build`      | `target/dx/drydrop-web/debug/web/public/`                                                                |
| CLI           | `mise run cli-build`      | `target/release/drydrop`                                                                                 |
| TUI           | `mise run tui-build`      | `target/release/drydrop-tui`                                                                             |
| Server        | `mise run server-build`   | `target/release/drydrop-server`                                                                          |
