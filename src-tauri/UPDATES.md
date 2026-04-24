# Auto-updater setup

v1.0 ships with `tauri-plugin-updater` wired up (no paid code-sign required).
Trust is established by a local minisign key pair; Tauri signs each update
with the private key, and the app verifies it against the embedded public
key before applying.

## One-time maintainer setup

1. **Generate the key pair** (already done once — don't regenerate unless the
   previous key is compromised, otherwise existing installs can't auto-update):

   ```bash
   cd demos/yt-extractor/lavescar-ytdl
   npx @tauri-apps/cli signer generate --ci -p "<STRONG_PASSWORD>" \
     -w ~/.tauri/lavescar-ytdl.key
   ```

   Output: `~/.tauri/lavescar-ytdl.key` (private) + `.pub` (public).

2. **Embed the public key** in `src-tauri/tauri.conf.json` under
   `plugins.updater.pubkey` (already done — matches the key generated on
   2026-04-25).

3. **Back up the private key**: copy `~/.tauri/lavescar-ytdl.key` into a
   password manager (1Password, KeePass, Bitwarden). Losing it means every
   installed copy of the app stops receiving auto-updates and users must
   manually re-download and reinstall.

4. **Add GitHub repository secrets** for CI signing:
   - `TAURI_SIGNING_PRIVATE_KEY`: full contents of the private key file
   - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`: the password you set in step 1

   These are consumed by `.github/workflows/release.yml` during `tauri build`.

## `latest.json` manifest

`tauri build` produces signed artifacts per platform (`.AppImage.tar.gz.sig`,
`.msi.zip.sig`, `.app.tar.gz.sig`). GitHub Releases needs a `latest.json`
manifest pointing installed clients at them. Format:

```json
{
  "version": "1.0.1",
  "notes": "see CHANGELOG",
  "pub_date": "2026-04-25T12:00:00Z",
  "platforms": {
    "linux-x86_64":    { "signature": "<base64>", "url": "https://github.com/.../latest/download/lavescar-ytdl_1.0.1_amd64.AppImage.tar.gz" },
    "windows-x86_64":  { "signature": "<base64>", "url": "https://github.com/.../latest/download/lavescar-ytdl_1.0.1_x64_en-US.msi.zip" },
    "darwin-aarch64":  { "signature": "<base64>", "url": "https://github.com/.../latest/download/lavescar-ytdl_1.0.1_aarch64.app.tar.gz" },
    "darwin-x86_64":   { "signature": "<base64>", "url": "https://github.com/.../latest/download/lavescar-ytdl_1.0.1_x64.app.tar.gz" }
  }
}
```

Construction can be automated with a small release step that reads each
`.sig` file (its contents are the signature) and emits the JSON. For v1.0
we treat this as a post-build manual step until we script it.

## Rotating keys (emergency)

If the private key is exposed:

1. Generate a brand-new keypair.
2. Replace `pubkey` in `tauri.conf.json`.
3. Cut a new release — **existing installs will not auto-upgrade** because
   they still verify against the old public key. Surface a manual-update
   banner via a release note and maintain a pinned GitHub issue.
