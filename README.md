# Instagram Desktop Native (Pure Rust)

An ultra-lightweight, high-performance standalone Windows desktop client for Instagram built entirely in **Rust** using `tao` and `wry` (native Edge WebView2 runtime).

## ✨ Features

- **True Single Standalone `.exe`**: Requires **NO** external DLLs, Python scripts, batch files, or companion folders. Run it anywhere!
- **Ultra-Low Memory Footprint**: Uses only **~30–40 MB of RAM** (compared to 400+ MB on Electron or standard Chromium browsers).
- **Embedded Official Icons**: Multi-resolution official Instagram vector ICO embedded directly in the Windows PE header and runtime taskbar.
- **Window Geometry Memory**: Automatically restores your preferred position, size, and maximize state.
- **Hardware-Accelerated Dark Canvas**: Pre-initializes a dark backdrop for zero flash upon launch.

## 🚀 Usage

Download the single executable `Instagram.exe` from [Releases](https://github.com/5ourav-dev/instagram-rust/releases) and double-click to launch:

```cmd
Instagram.exe
```

## 🛠️ Building from Source

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) (stable channel)
- Windows 10 / 11 with Microsoft Edge WebView2 runtime (pre-installed by default)
- MinGW `windres` (for embedding the `.rc` icon)

### Build Command

```cmd
cargo build --release
```

The compiled standalone executable will be located in:
```
target/release/instagram-rust.exe
```

