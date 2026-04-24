# Changelog

## 1.0.0 — 2026-04-25

First stable release. Built on top of the v0.1.0-alpha internal cut (Tauri 2
scaffold + yt-dlp sidecar + SQLite + 5 views + settings). This release is the
hardening pass that closes the gaps surfaced during the first real-world run.

### Added
- **True concurrency gating.** `DownloadManager` now holds a `tokio::sync::Semaphore`
  whose permit count tracks the `concurrent_limit` setting. Raising/lowering the
  cap at runtime is non-blocking: new slots open immediately, shrinks burn
  permits in the background so in-flight tasks finish at their old rate.
- **yt-dlp self-update** in Settings. Streams the latest GitHub release into
  `app_data_dir/binaries/`, computes a running SHA-256, and does an atomic swap;
  the bundled sidecar stays untouched as a rollback target.
- **Playlist inspection.** New `inspect_url` command detects `_type: "playlist"`
  via `--flat-playlist --dump-single-json`. Playlist URLs now open a
  `PlaylistModal` (all / range / manual selection) and enqueue each entry as an
  independent download sharing the active preset.
- **Real cancel semantics.** Cancelling moves the row to History with a
  `cancelled` status instead of vanishing; the startup orphan-`.part` scan
  surfaces any leftover fragments with a one-click "delete all" banner.
- **Categorised error UX.** `YtdlpError` serializes as `{kind, message}`; the
  new `ErrorToast` maps kinds (`geo_blocked`, `auth_required`, `not_found`,
  `network`, `io`, …) to actionable suggestions. `auth_required` toast jumps
  straight to the Cookies view.
- **Clipboard watcher.** Opt-in background task polls the clipboard every 1s,
  matches known video hosts (YouTube / Twitch / Vimeo / Soundcloud / Bandcamp /
  Dailymotion / Bilibili / Niconico), and offers a "fetch this?" prompt above
  the URL bar.
- **Per-download subtitle picker.** New `SubtitleModal` lets you pick any subset
  of available languages, choose manual vs auto-generated, and embed vs sidecar
  `.vtt` files. Flags forwarded to `yt-dlp` as `--write-subs --sub-langs …
  --embed-subs` / `--write-auto-subs`.
- **Auto-updater.** Bundled `tauri-plugin-updater` with a local minisign
  keypair. CI release builds produce signed `.sig` artifacts; the app checks
  on startup and shows an `UpdateBanner` when a new release is available.
- **Accessibility pass.** Sidebar is a real `role="tablist"` with arrow-key
  navigation, `aria-selected`, and visually hidden ARIA labels. All modals are
  `role="dialog"` with `aria-modal` and `aria-labelledby`.
- **Rust test suite.** 26 unit tests cover yt-dlp JSON parsing, progress-line
  decoding, error categorisation, clipboard URL detection, SQLite round-trips,
  and preset CRUD. Frontend runs vitest (jsdom) against the error-toast store.
- **CI workflow.** `ci.yml` runs `cargo fmt --check`, `cargo clippy -D
  warnings`, `cargo test`, `npm run check`, and `npm test` on every PR and push
  to `main`. `release.yml` matrix-builds Linux/Windows/macOS bundles and
  attaches them as a draft release.
- **Redesigned icon.** No more placeholder glyph — amber-on-dark minimalist
  mark built at 1024×1024, regenerated for every platform target.
- **Manual test matrix.** `docs/test-matrix.md` captures the scenarios that
  automated tests can't cover (real URLs, geo-blocking, playlists, cookies).

### Changed
- Runtime throttle now correctly reflects the latest setting for every new
  download. The Settings modal says "applies to new downloads" — active tasks
  finish at the rate they started with (yt-dlp doesn't hot-reload
  `--limit-rate`).
- `DownloadRequest.flags` carries preset extras (SponsorBlock, split-chapters,
  embed-metadata, …) end-to-end instead of being dropped at the boundary.
- `HistoryView` groups by day, filters `cancelled` alongside `done` / `error`.
- `ActiveDownloads` shows only in-flight states (active / queued / paused),
  matching what the panel name implies.

### Removed
- Unused `YtdlpError::NonZeroExit` variant (never constructed anywhere).

### Security
- Auto-updater signatures verified against an embedded minisign public key
  before install. Without a matching signature the update is rejected.

### Known limitations
- **Unsigned bundles.** macOS and Windows builds are not code-signed (no paid
  developer certificate). README documents the Gatekeeper / SmartScreen bypass
  steps required on first launch.
- **CI workflow untested against a real tag.** The YAML passes a linter pass
  locally but needs a `v0.99.0-rc1` push to validate end-to-end artifact
  production.
- **`latest.json` manifest generation is manual.** See
  `src-tauri/UPDATES.md` for the construction steps; scripting this into the
  release workflow is deferred to v1.1.
- **a11y audit is partial.** Sidebar + overlays have ARIA roles; a full
  axe-core CI pass has not been wired up (deferred).
