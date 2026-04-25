# Cirrus CI setup (replaces locked GitHub Actions)

GitHub Actions on `Lavescar-dev` is currently blocked by a payment-method
authorization issue. While that's pending, multi-platform release builds
run on **Cirrus CI**, which has its own free tier for public repos and
supports native macOS + Windows runners (which we need — Tauri can't
cross-compile reliably from Linux).

`.cirrus.yml` is already in this repo and triggers on tag push (`v*`).
It builds Linux/Windows/macOS bundles in parallel and appends them to the
matching GitHub Release.

## One-time setup

1. **Sign in:** open <https://cirrus-ci.com/> and click *Sign in with
   GitHub*. Grant the requested permissions (read access to repo events).

2. **Install the Cirrus app on the repo:** during sign-in or from
   <https://github.com/marketplace/cirrus-ci>, click *Install* → choose
   *Lavescar-dev/lavescar-ytdl* (only this repo, not all). Cirrus will
   start watching for tag pushes.

3. **Add three repository secrets** in Cirrus:
   <https://cirrus-ci.com/settings/repository/Lavescar-dev/lavescar-ytdl>
   → *Encrypted Variables*:

   | Variable                              | Value                                                                              |
   |---------------------------------------|------------------------------------------------------------------------------------|
   | `GH_TOKEN`                            | A GitHub Personal Access Token with `repo` scope. Used by `gh release upload`.    |
   | `TAURI_SIGNING_PRIVATE_KEY`           | Full contents of `~/.tauri/lavescar-ytdl.key` (the private minisign key).         |
   | `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`  | Empty string (we generated the key without a password).                            |

   To generate a PAT: <https://github.com/settings/tokens/new> → *generate
   new token (classic)* → name `cirrus-release`, scope `repo`. Copy the
   `ghp_…` value into the Cirrus secret form.

## Trigger a release build

```bash
git tag v1.0.0
git push lavescar-ytdl v1.0.0
```

Cirrus picks the tag up immediately. Build progress is visible at
<https://cirrus-ci.com/github/Lavescar-dev/lavescar-ytdl>.

Each platform finishes independently and uploads its artifacts to the
GitHub Release for that tag (Cirrus uses `gh release upload --clobber`,
so re-running a task replaces, not duplicates). When the run completes:

- `lavescar-ytdl_VERSION_amd64.AppImage` + `.deb` (Linux)
- `lavescar-ytdl_VERSION_x64_en-US.msi` + setup.exe (Windows)
- `lavescar-ytdl_VERSION_aarch64.dmg` + `.app.tar.gz` + `.app.tar.gz.sig`
  (macOS)

## Falling back to GitHub Actions later

When the Lavescar-dev billing lock is cleared, `.github/workflows/release.yml`
already exists and produces the same set of artifacts via GitHub's runners.
You can delete `.cirrus.yml` then, or keep both — Cirrus only runs on
tag pushes, so nothing fires on regular commits.

## Notes

- **Cirrus free tier limits**: public repos have generous community
  credits — for a Tauri build matrix this is comfortably under the cap.
  See <https://cirrus-ci.org/pricing/> if it ever changes.
- **The first run will likely fail somewhere** — fixing CI yaml on the
  first attempt is rare. Watch the run logs, iterate the yaml, push a
  fresh tag (`v1.0.1-rc1` etc.). Once it's green, cut the real `v1.0.0`.
- **Linux artifact already produced locally**: this repo's maintainer
  also runs `npm run tauri:build` on a Linux machine and uploads the
  AppImage + deb directly via `gh release upload`. So even if Cirrus
  Linux build fails, Linux users still have something to download.
