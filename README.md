# lavescar ▸ yt-dlp

yt-dlp için opinionated desktop frontend — Tauri 2 + SvelteKit (SPA) + Svelte 5 runes.

![preview](docs/preview.png)

## Features

- **Keyboard-first** — `⌘L` focus URL, `⌘1/2/3` hotkey presets, `⌘,` settings, `?` cheatsheet
- **Preset system** — yt-dlp `-f` spec + extra flags (SponsorBlock, chapter split, embed metadata), per-preset hotkey
- **Queue + history** — SQLite-backed, re-download guard, date-grouped history with search
- **Progress stream** — real-time bytes/speed/ETA via Tauri events (no polling)
- **Cookie support** — `--cookies-from-browser firefox|chromium|brave` direct from settings
- **Throttle** — per-download `--limit-rate` MB/s slider
- **Open in mpv** — or OS default player; "show in folder" reveal
- **Dependency detection** — ffmpeg/aria2c version inspection in settings

## Install

Prebuilt bundles on [Releases](https://github.com/lavescar-dev/lavescar-ytdl/releases)
— v1.0.0 ships Linux AppImage/deb, Windows MSI/NSIS, macOS dmg.

**The bundles are not code-signed** (no paid Apple / Windows certs). The app
is signed for auto-updates only — OS install prompts will show an "unknown
developer" warning on first launch. Steps below.

### Linux

No warning, but you must mark the AppImage executable:

```bash
chmod +x lavescar-ytdl_*.AppImage
./lavescar-ytdl_*.AppImage

# or as a deb
sudo dpkg -i lavescar-ytdl_*.deb
```

### macOS

On first launch Gatekeeper will refuse. Two options:

1. **Right-click → Open → Open anyway** (per-app bypass).
2. **Strip the quarantine attribute** (permanent):
   ```bash
   xattr -rd com.apple.quarantine /Applications/lavescar-ytdl.app
   ```

### Windows

SmartScreen will say "Windows protected your PC". Click **More info** →
**Run anyway**.

### Verifying downloads (optional but recommended)

Each release includes SHA-256 hashes. On any platform:

```bash
sha256sum lavescar-ytdl_*.AppImage   # Linux
shasum -a 256 lavescar-ytdl_*.dmg    # macOS
certutil -hashfile lavescar-ytdl_*.msi SHA256   # Windows
```

Match the output against the `SHASUMS.txt` asset on the release page.

## Build from source

Prereqs: Rust 1.77.2+, Node 20+, platform Tauri deps ([tauri.app/start/prerequisites](https://tauri.app/start/prerequisites/)).

```bash
# clone and enter
cd demos/yt-extractor/lavescar-ytdl

# fetch yt-dlp sidecar for your platform
cd src-tauri/binaries
curl -L -o yt-dlp-x86_64-unknown-linux-gnu https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux
chmod +x yt-dlp-x86_64-unknown-linux-gnu
cd ../..

# install frontend + run
npm install
npm run tauri:dev          # dev window with hot reload
npm run tauri:build        # production bundle
```

### Cross-platform sidecar names

yt-dlp binaries must live in `src-tauri/binaries/` with Tauri's target-triple suffix:

| Platform         | Filename                                |
|------------------|-----------------------------------------|
| Linux x86_64     | `yt-dlp-x86_64-unknown-linux-gnu`       |
| Windows x86_64   | `yt-dlp-x86_64-pc-windows-msvc.exe`     |
| macOS Apple Silicon | `yt-dlp-aarch64-apple-darwin`        |
| macOS Intel      | `yt-dlp-x86_64-apple-darwin`            |

CI downloads them automatically — see `.github/workflows/release.yml`.

## Keyboard shortcuts

| Key     | Action                         |
|---------|--------------------------------|
| `⌘L`    | Focus URL input                |
| `⌘1-3`  | Select hotkey-bound preset     |
| `⌘,`    | Open settings                  |
| `?`     | Show shortcuts overlay         |
| `ESC`   | Close overlay                  |
| `↵`     | Fetch metadata (URL input)     |

## Architecture

- **Frontend** — SvelteKit (SPA mode, `adapter-static` + `fallback: 'index.html'`). Svelte 5 rune-based stores (`.svelte.ts` singletons).
- **Backend** — Tauri 2, `tauri-plugin-shell` for yt-dlp sidecar, `tauri-plugin-dialog` for folder picker.
- **Persistence** — `rusqlite` bundled SQLite, migrations under `src-tauri/migrations/`.
- **Single boundary** — `src/lib/api/tauri.ts` auto-detects Tauri vs browser; mock data keeps dev server standalone.

## Legal

Licensed under AGPL-3.0 (see `LICENSE`). Bundled yt-dlp is Unlicense/public domain. ffmpeg (when bundled) is LGPL/GPL.

**Use responsibly.** YouTube Terms of Service prohibit downloads without permission; this tool is intended for content you have rights to, public domain material, or personal archival where legally permitted. You are responsible for your use.

## Roadmap (v1.1+)

Shipped in v1.0:

- Concurrent-limit semaphore (true gating, live-adjustable)
- yt-dlp self-update from Settings
- Playlist modal (all / range / manual)
- Subtitle per-download picker
- Clipboard URL auto-detect (opt-in)
- Categorised error toasts with actionable suggestions
- Auto-updater with local minisign signing
- Cancel → `.part` cleanup, startup orphan sweep

Deferred to v1.1+:

- Twitch VOD / Soundcloud / Bandcamp preset categories
- Batch URL file import (`.txt` → enqueue)
- Post-process hooks (custom scripts on done)
- Firefox/Chromium direct cookie import (SQLite decrypt via DPAPI /
  Keychain / libsecret — v1.0 uses yt-dlp's `--cookies-from-browser`)
- `latest.json` manifest automation in CI (currently constructed by hand)
- axe-core a11y lint in CI
- Paid code signing (Apple Developer + Windows EV)
- Remote control HTTP API (headless queue from another device)
- CLI mode (`lavescar-ytdl --url … --preset archive-av1`)
