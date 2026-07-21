#!/usr/bin/env node
'use strict';

// Launcher for the `nuwax-grok-build` CLI.
//
// The native binary is shipped in per-platform packages declared as
// optionalDependencies. Each platform package's `main` exports the absolute
// path to its bundled `grok` executable. We resolve the right one for the
// current process, then spawn it with stdio inherited so the TUI behaves
// exactly like a direct invocation.

const { spawn } = require('node:child_process');

/** Maps Node's `${process.platform}-${process.arch}` to the npm sub-package. */
const PLATFORM_PACKAGES = {
  'darwin-arm64': 'nuwax-grok-build-darwin-arm64',
  'darwin-x64': 'nuwax-grok-build-darwin-x64',
  'linux-arm64': 'nuwax-grok-build-linux-arm64',
  'linux-x64': 'nuwax-grok-build-linux-x64',
  'win32-x64': 'nuwax-grok-build-win32-x64',
};

const platformKey = `${process.platform}-${process.arch}`;
const pkgName = PLATFORM_PACKAGES[platformKey];

if (!pkgName) {
  console.error(`nuwax-grok-build: no prebuilt binary for ${platformKey}.`);
  console.error('Supported: darwin-arm64, darwin-x64, linux-arm64, linux-x64, win32-x64.');
  process.exit(1);
}

let binPath;
try {
  // Requiring the package runs its index.js, which returns the binary path.
  binPath = require(pkgName);
} catch (err) {
  console.error(`nuwax-grok-build: platform binary "${pkgName}" could not be loaded.`);
  console.error('This usually means it was not installed (e.g. --ignore-scripts, --omit=optional,');
  console.error('or a corporate npm mirror missing the package). Reinstall without those flags.');
  if (err && err.message) {
    console.error(`Underlying error: ${err.message}`);
  }
  process.exit(1);
}

const child = spawn(binPath, process.argv.slice(2), { stdio: 'inherit' });

child.on('error', (err) => {
  console.error(`nuwax-grok-build: failed to launch binary at ${binPath}: ${err.message}`);
  process.exit(1);
});

child.on('exit', (code, signal) => {
  if (signal) {
    // Mirror the conventional "killed by signal" exit code.
    process.exit(130);
  }
  process.exit(code == null ? 1 : code);
});
