# Voidium 1.1

Voidium is a non-Chromium browser project with a small native core and a custom HTML/CSS/JS interface.

This upgrade fixes the previous release pipeline and redesigns the desktop UI for DPI scaling, narrow windows, high-DPI screens, and responsive layouts.

Added:
- Responsive browser shell with tabs and address bar.
- Better 100+ setting search and grouped settings.
- Settings persistence with localStorage.
- Optimizer panel for safe browser-side performance settings.
- DPI-aware CSS using viewport units and clamp().
- Mobile/narrow-window fallbacks.
- Cleaner dark UI with reduced visual clutter.
- Windows x64 installer workflow.

The current native core remains an early browser engine foundation. It does not yet implement full HTML/CSS/JavaScript compatibility like mature browsers.
