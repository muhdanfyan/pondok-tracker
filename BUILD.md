# Build Guide for Pondok Tracker

This guide covers how to set up your environment and build the application for macOS, Windows, and Linux.

## 1. Prerequisites

This application is built using [Tauri](https://tauri.app/), which requires both Node.js and Rust.

### Install Node.js
You already have Node.js installed.
```bash
node --version
npm --version
```

### Install Rust (Required)
Your system currently lacks the Rust toolchain. Install it by running:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Restart your terminal after installation to ensure `cargo` is in your PATH.

## 2. Development

To run the application locally in development mode:

```bash
npm install
npm run tauri:dev
```

## 3. Building for Your Platform (macOS)

Since you are on macOS, you can build the native macOS application (.dmg, .app) directly.

```bash
npm run tauri:build
```
The output will be located in: `src-tauri/target/release/bundle/macos/` or `dmg/`.

## 4. Cross-Platform Building

Tauri applications compile to native binaries. **You cannot build a Windows `.exe` or Linux AppImage directly from macOS** without using a virtual machine or a CI/CD pipeline (recommended).

### Option A: GitHub Actions (Recommended)
The easiest way to build for all platforms is to use GitHub Actions.

1. Push your code to a GitHub repository.
2. Create a workflow file at `.github/workflows/release.yml`.
3. Use the official Tauri Action.

**Example `.github/workflows/release.yml`:**
```yaml
name: Release
on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

jobs:
  release:
    permissions:
      contents: write
    strategy:
      fail-fast: false
      matrix:
        platform: [macos-latest, ubuntu-20.04, windows-latest]
    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v3
      - name: setup node
        uses: actions/setup-node@v3
        with:
          node-version: 20
      - name: install Rust stable
        uses: dtolnay/rust-toolchain@stable
      - name: install dependencies (ubuntu only)
        if: matrix.platform == 'ubuntu-20.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
      - name: install frontend dependencies
        run: npm install
      - uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tagName: ${{ github.ref_name }}
          releaseName: 'Pondok Tracker v__VERSION__'
          releaseBody: 'See the assets to download this version and install.'
          releaseDraft: true
          prerelease: false
```

### Option B: Local Virtual Machines
If you strictly need to build locally:
- **Windows**: Use Parallels Desktop, VMware Fusion, or Boot Camp (Intel Macs) to run Windows. Install Node.js and Rust on Windows, then run `npm run tauri:build`.
- **Linux**: Use Docker (complex for GUI apps) or a Linux VM (VirtualBox/Parallels). Install build tools, Node.js, Rust, then run `npm run tauri:build`.

## 5. Troubleshooting Common Issues
- **"Command not found: cargo"**: Ensure you've installed Rust and restarted your terminal.
- **Permission Denied**: Check that you have permission to write to the `src-tauri/target` directory.
