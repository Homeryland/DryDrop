# 构建 DryDrop

本文档介绍各平台的构建命令以及构建产物的存放位置。

## 前置准备

```bash
mise install                 # 安装固定版本的工具链（Rust、dx、pnpm 等）
mise bootstrap               # rustup 目标 + Android NDK + pnpm install + Tailwind CSS
```

> 构建任何 UI 应用之前，必须先生成 Tailwind CSS 资产
> （`crates/dioxus-components/assets/tailwind.css`），`mise bootstrap` 会完成这一步。

## 桌面端

### macOS

在仓库根目录执行：

```bash
mise run macos-build
```

该命令会在 `apps/desktop` 中运行 `dx bundle --macos --package-types "macos"
--package-types "dmg"`。

构建产物：

```
target/dx/drydrop-desktop/bundle/macos/macos/
├── DryDrop Desktop.app
└── DryDrop Desktop_<version>_aarch64.dmg
```

### Windows

TODO

## 移动端

> 注意：当前的 `dx bundle` 任务构建的是 **debug** 产物（未加 `--release`）。

### iOS

在仓库根目录执行：

```bash
mise run ios-build
```

该命令会在 `apps/mobile` 中运行 `dx bundle --ios --package-types "ios"`，并将
应用包重命名为 `DryDrop Mobile.app`。

构建产物：

```
target/dx/drydrop-mobile/debug/ios/DryDrop Mobile.app
```

### Android

在仓库根目录执行：

```bash
mise run android-build
```

该命令会在 `apps/mobile` 中运行 `dx bundle --android --package-types "apk"`，
并将 APK 重命名为 `DryDrop Mobile.apk`。

构建产物：

```
target/dx/drydrop-mobile/debug/android/app/app/build/outputs/apk/debug/DryDrop Mobile.apk
```

## Web

在仓库根目录执行：

```bash
mise run web-build
```

该命令会在 `apps/web` 中运行 `dx bundle --web`。

构建产物（静态站点 —— 部署 `public` 目录即可）：

```
target/dx/drydrop-web/debug/web/public/
```

（加上 `--release` 即可得到优化构建，产物位于
`target/dx/drydrop-web/release/web/public/`。）

## CLI、TUI、Server

在仓库根目录执行：

```bash
mise run cli-build         # CLI  （二进制名：drydrop）
mise run tui-build         # TUI  （二进制名：drydrop-tui）
mise run server-build      # Server（二进制名：drydrop-server）
```

这些命令会在对应的应用目录中运行 `mbx build --release`（`mbx` 与 `cargo`
可互换，见 `mise.toml`）。

构建产物：

| 应用   | 二进制文件                   |
|--------|------------------------------|
| CLI    | `target/release/drydrop`     |
| TUI    | `target/release/drydrop-tui` |
| Server | `target/release/drydrop-server` |

## 产物位置一览

| 平台           | 构建命令                        | 产物位置                                                                                       |
|----------------|---------------------------------|------------------------------------------------------------------------------------------------|
| 桌面 macOS     | `mise run macos-build`          | `target/dx/drydrop-desktop/bundle/macos/macos/`                                                 |
| 桌面 Windows   | —（TODO）                       | —                                                                                               |
| iOS            | `mise run ios-build`            | `target/dx/drydrop-mobile/debug/ios/DryDrop Mobile.app`                                         |
| Android        | `mise run android-build`        | `target/dx/drydrop-mobile/debug/android/app/app/build/outputs/apk/debug/DryDrop Mobile.apk`     |
| Web            | `mise run web-build`            | `target/dx/drydrop-web/debug/web/public/`                                                       |
| CLI            | `mise run cli-build`            | `target/release/drydrop`                                                                        |
| TUI            | `mise run tui-build`            | `target/release/drydrop-tui`                                                                    |
| Server         | `mise run server-build`         | `target/release/drydrop-server`                                                                 |
