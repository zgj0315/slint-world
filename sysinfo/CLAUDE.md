# CLAUDE.md

本文件为 Claude Code（claude.ai/code）在此代码仓库中工作时提供指导。

## 项目简介

基于 Slint（Rust UI 框架）构建的跨平台系统信息应用，支持桌面端和 Android 端运行。在 Android 上，通过 JNI 查询设备信息和 Wi-Fi 详情。

## 常用命令

### 桌面端
- `cargo run` — 在桌面端运行
- `cargo build --release` — 构建发布版本
- `cargo packager -r` — 打包桌面端应用

### Android 端
- `x devices` — 列出已连接的 Android 设备
- `x run --device adb:<device-id>` — 部署到指定 Android 设备
- `x build --platform android --arch arm64 --format apk --release` — 构建 Android APK

## 架构说明

- `src/main.rs` — 入口点；配置 Windows 子系统，调用 `sysinfo::run()`
- `src/lib.rs` — 核心逻辑；`run()` 创建 AppWindow，获取系统信息并绑定到 UI
- `ui/app-window.slint` — Slint UI 定义；包含标题和信息文本的单窗口
- `build.rs` — 通过 `slint_build::compile()` 编译 `ui/app-window.slint`
- `manifest.yaml` — Android 应用清单（包名、权限：Wi-Fi、网络、位置）

### 平台分支
`lib.rs` 使用 `#[cfg(target_os = "android")]` 区分不同平台行为：
- **Android**：`get_android_system_info()` 通过 JNI（`jni` crate + `ndk-context`）调用 Android API，获取制造商、型号、主板、Android 版本、Wi-Fi SSID、RSSI、链路速度和 IP 地址。
- **桌面端**：返回静态字符串 "Not running on Android"。

### Crate 配置
该 crate 同时编译为 `cdylib`（用于 Android）和 `rlib`（用于桌面端）。`slint` 依赖在 Android 目标上启用 `android` feature。
