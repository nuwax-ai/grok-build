# npm distribution for `@nuwax-ai/nuwax-grok`

This directory holds the npm packaging layer for the Grok Build CLI. The CLI
itself is a native Rust binary (`xai-grok-pager`). Upstream ships it as `grok`;
this npm package exposes it as the **`nuwax-grok`** command so it can coexist
with an official `grok` install (e.g. via Homebrew) without a command-name clash.

> **End-user guide:** [USAGE.md](USAGE.md) — install via npm and use a custom LLM
> (DeepSeek, OpenAI, Anthropic, Ollama, …) inside Zed / VS Code / Cursor.

## Layout

```
npm/
  nuwax-grok/                          # main package (npm name: @nuwax-ai/nuwax-grok)
    package.json                    #   bin.nuwax-grok + optionalDependencies
    bin.js                          #   picks the right platform binary, spawns it
  nuwax-grok-<os>-<arch>/              # platform sub-package (npm name: @nuwax-ai/nuwax-grok-<os>-<arch>)
    package.json                    #   os/cpu constraints so npm only fetches a match
    index.js                        #   exports the absolute path to the bundled binary
    bin/                            #   (filled by CI with the compiled `grok` binary)
  scripts/
    set-version.mjs                 # stamps the release version into every package.json
```

Platform sub-packages: `linux-x64`, `linux-arm64`, `darwin-x64`, `darwin-arm64`,
`win32-x64`.

## How it works

1. `npm install -g @nuwax-ai/nuwax-grok` installs the main package plus, as an
   **optional dependency**, exactly the one platform sub-package matching the
   user's OS/arch. npm skips the others thanks to the `os`/`cpu` fields, so the
   install stays small and works offline (no postinstall download).
2. Running `nuwax-grok` executes `bin.js`, which requires the platform sub-package
   to get its binary path and spawns it with inherited stdio — the TUI behaves
   exactly like a direct invocation.

## Releasing

Releases are **tag-triggered** (the git tag is the single source of truth for
the version). There are two pipelines under `.github/workflows/`:

| Tag                   | Workflow              | npm dist-tag | Install                              |
|-----------------------|-----------------------|--------------|--------------------------------------|
| `v0.2.107-beta.1`     | `publish-beta.yml`    | `beta`       | `npm i -g @nuwax-ai/nuwax-grok@beta`     |
| `v0.2.107`            | `publish-stable.yml`  | `latest`     | `npm i -g @nuwax-ai/nuwax-grok`          |

Both call the reusable `_build-and-publish.yml`, which:

1. Derives the version from the tag (strips the leading `v`) and bakes it into
   the binary via `GROK_VERSION`.
2. Builds `cargo build -p xai-grok-pager-bin --release` on one runner per
   platform, stages each binary into its sub-package, and `npm publish`es it.
3. Publishes the main package last, once all platform packages exist.

### First-time setup

- Create an npm **automation/granular access token** with publish rights for
  `@nuwax-ai/nuwax-grok` and the five `@nuwax-ai/nuwax-grok-*` names, and add it as the
  `NPM_TOKEN` repository (or organization) secret.
- Confirm the six package names are available on the registry before first push.
- Cut a tag and push it: `git tag v0.2.107-beta.1 && git push origin v0.2.107-beta.1`.

### Notes / caveats

- **Windows is best-effort.** Its matrix leg uses `continue-on-error`, so a
  Windows build failure does not block the macOS/Linux release. Until it builds
  cleanly, Windows users get an optional-dependency warning and no binary.
- **Linux arm64** builds natively on `ubuntu-24.04-arm` (free for public repos).
  If that runner is unavailable, switch that leg to `ubuntu-24.04` and
  cross-compile with `cross` (or a `gcc-aarch64-linux-gnu` linker) instead.
- `cargo build --locked` requires the committed `Cargo.lock` to match the
  generated workspace. If a release fails on the lockfile, drop `--locked`.
- `protoc` is required at build time (strict under `GITHUB_ACTIONS=true`); the
  workflow installs it via `arduino/setup-protoc`.
