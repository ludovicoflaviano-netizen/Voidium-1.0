# Voidium 1.0

Voidium is a lightweight browser project designed without Chromium. The 1.0 foundation uses Rust for the native core and Reqwest with Rustls for HTTPS transport.

Current foundation:
- HTTPS-only navigation.
- Rustls TLS rather than Chromium networking.
- No telemetry, VPN, bundled ad network, or account requirement.
- Persistent settings in the platform data directory.
- Search and basic HTML-to-text viewing.
- Developer-mode foundation.
- A 100+ setting catalog is provided in ui/settings.js.
- GitHub Actions builds Windows x64 and packages an Inno Setup installer.

A complete modern browser engine requires HTML parsing, CSS layout, JavaScript, DOM, graphics, media, accessibility, storage, sandboxing, and strong site compatibility. Recreating that engine from scratch is a multi-year project. Voidium keeps the native browser core separate from the UI and avoids Chromium while the engine grows.

Build:
cargo run
cargo build --release

Windows releases are built by .github/workflows/windows.yml.
