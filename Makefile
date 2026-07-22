# Grok Build — local development / testing helpers.
#
# These targets build and install the `xai-grok-pager` binary from your local
# source into ~/.cargo/bin, so an editor config (Zed / VS Code) pointing at the
# `xai-grok-pager` command picks up your latest changes — no need to wait for
# the npm CI publish to test. Requires `protoc` on PATH (or `bin/protoc`).

PAGER_BIN_CRATE := crates/codegen/xai-grok-pager-bin

# Bundle the local ripgrep (if present) so the xai-grok-shell build script does
# NOT fetch rg from GitHub during the build (fails offline / behind a proxy).
# Falls back to the download when rg is not on PATH.
RG_PATH := $(shell command -v rg 2>/dev/null)
RG_ENV := $(if $(RG_PATH),GROK_SHELL_BUNDLE_RG_PATH=$(RG_PATH))

# Install the locally-built binary to ~/.cargo/bin as `xai-grok-pager`.
# `--force` reinstalls even when the version is unchanged, so iterative
# source edits are picked up without bumping the version.
.PHONY: install-local
install-local:
	$(RG_ENV) cargo install --path $(PAGER_BIN_CRATE) --locked --force
	@echo ''
	@echo '✓ Installed xai-grok-pager to ~/.cargo/bin/'
	@echo '  Verify:      xai-grok-pager -V'
	@echo '  Editor cmd:  "command": "xai-grok-pager", "args": ["agent", "stdio"]'
	@echo '  Debug logs:  ~/.grok/debug/latest.txt   (when GROK_DEBUG_LOG=1)'

# Fast type-check of the binary crate (no codegen) — quicker than install-local.
.PHONY: check
check:
	$(RG_ENV) cargo check -p xai-grok-pager-bin --locked

# Release build of the binary into target/release/ (without installing).
.PHONY: build
build:
	$(RG_ENV) cargo build --release -p xai-grok-pager-bin --locked

# Remove the locally-installed binary.
.PHONY: uninstall-local
uninstall-local:
	cargo uninstall xai-grok-pager || true
