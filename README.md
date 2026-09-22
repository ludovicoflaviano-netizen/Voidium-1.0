# Voidium 1.0

Voidium is a privacy-focused desktop browser shell built around Servo/Verso instead of Chromium. The UI is plain HTML, CSS and JavaScript. The native layer is Rust.

## Features
- Chromium-free browser engine architecture
- HTTPS-first navigation
- Private local settings with no telemetry service
- Brave-style tabs and address/search bar
- Searchable settings with 100+ preferences
- Developer panel for page HTML/CSS/JavaScript work
- Windows NSIS installer through Tauri
- Release build caching in GitHub Actions

## Build
Install Rust, Visual Studio 2022 C++ build tools, the Windows SDK and Git.

```powershell
cargo install tauri-cli --version ^2
git clone https://github.com/ludovicoflaviano-netizen/Voidium-1.0.git
cd Voidium-1.0
cargo tauri build
```

The project uses Tauri Runtime Verso, which embeds the Servo browser engine rather than Chromium/WebView2. Servo currently supports Windows, macOS and Linux, but its embedding API is still evolving, so the engine adapter is isolated from the UI.

Voidium intentionally does not include a VPN. HTTP compatibility is disabled by default.

License: MPL-2.0.