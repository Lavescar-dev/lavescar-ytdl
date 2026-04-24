# Manual test matrix — v1.0

Run through this checklist before tagging a release. Each row is a scenario;
mark ✓ or ✗ in the status column and jot the date/build you tested against.

_Automated coverage (cargo + vitest) lives under `src-tauri/src/**/tests` and
`src/**/*.test.ts`; this matrix covers behaviours that need a real window._

## Core flow

| Scenario | Steps | Expected | Status |
|---|---|---|---|
| Single-video fetch | Paste a public YouTube URL, press Enter | MetadataPanel populates with title, codec chips, format list | |
| Queue + download | After fetch, press "▸ queue" | Row appears in ACTIVE; progress bar animates; file lands in output dir | |
| Cancel mid-flight | Click ✕ on an active row | Row disappears from ACTIVE; History shows "cancelled"; `.part` cleaned up (or OrphanPartsBanner offers cleanup on next launch) | |
| History search | Complete 3 downloads, open History, type in search box | List filters by title/url/codec | |
| Re-download guard | Queue the same URL twice | Second attempt emits `download:already` event; user can still force re-download | |

## Concurrent + throttle

| Scenario | Steps | Expected | Status |
|---|---|---|---|
| Concurrent cap | Settings → limit=2, queue 5 URLs | Only 2 active at a time; queued rows shown; next one starts as an active finishes | |
| Raise limit live | While 2 are running at limit=2, set limit=5 in Settings, queue 3 more | All 3 start immediately | |
| Lower limit live | Running 5 with limit=5, set limit=2 | Running tasks keep going; new queued items wait until ≤2 active | |
| Throttle | Settings → throttle=5 MB/s, queue a large video | Progress rate caps near 5 MB/s | |

## yt-dlp edge cases

| URL type | Expected UX | Status |
|---|---|---|
| Geo-blocked | ErrorToast `geo_blocked` with VPN/cookies suggestion; History row → error | |
| Age-gated | ErrorToast `auth_required`; banner button jumps to Cookies view | |
| Removed / private video | ErrorToast `not_found`; History → error | |
| Public playlist | PlaylistModal opens with entries + range input; selecting 3 queues 3 downloads | |
| Livestream | yt-dlp default behaviour; UI shows row with unknown duration; error propagates cleanly if unsupported | |
| 4K / 10+ GB file | Progress bar correct, no overflow; final file size matches bytes | |
| Network drop mid-download | yt-dlp retries internally; row stays "active" | |
| Disk full | Error surfaces as `io` or `unknown`; row in history marked error | |

## Integrations

| Scenario | Steps | Expected | Status |
|---|---|---|---|
| yt-dlp self-update | Settings → "▸ update yt-dlp" | Progress bar streams; runtime info reflects new tag; subsequent downloads use new binary | |
| Cookies from browser | Settings → cookies_source=Firefox, queue an age-gated video | yt-dlp adds `--cookies-from-browser firefox`; download succeeds | |
| Clipboard listener | Enable in Settings, copy a YouTube URL to clipboard | ClipboardPrompt appears within ~1s; "fetch" pre-fills metadata | |
| Subtitle picker | For a multi-language video, open subtitle modal, pick EN manual + TR auto, queue | Downloaded file has the subtitles (embedded or sidecar `.vtt`) | |
| mpv open | Completed row → "▸ open in mpv" | mpv launches with the file; falls back to OS default if mpv missing | |
| Show in folder | Completed row → folder glyph | File manager opens with the file selected | |

## Keyboard + a11y

| Action | Keys | Status |
|---|---|---|
| Focus URL input | `⌘L` / Ctrl+L | |
| Hotkey preset switch | `⌘1` / `⌘2` / `⌘3` | |
| Open settings | `⌘,` / Ctrl+, | |
| Shortcuts cheatsheet | `?` | |
| Close modal/overlay | `ESC` | |
| Sidebar nav with arrows | `↑` / `↓` after focusing sidebar | |
| Sidebar activate | `Enter` / `Space` | |

## Release hygiene

| Item | Status |
|---|---|
| `cargo fmt --check` clean | |
| `cargo clippy --all-targets -- -D warnings` clean | |
| `cargo test` all green | |
| `npm run check` 0 errors, 0 warnings | |
| `npm test` all green | |
| CI workflow green on push | |
| Fresh install launches without Gatekeeper/SmartScreen surprises (docs mention bypass) | |
| `latest.json` manifest present on release | |
| Version bumped in `package.json` + `src-tauri/Cargo.toml` + `tauri.conf.json` | |
