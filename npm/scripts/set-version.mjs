#!/usr/bin/env node
// Injects a release version (derived from the git tag by CI) into every npm
// package.json under npm/. The main package's `optionalDependencies` entries
// for the platform sub-packages are kept in lock-step with the same version.
//
// Usage:  node npm/scripts/set-version.mjs <version>   e.g. 0.2.107  or  0.2.107-beta.1

import { readFileSync, writeFileSync, readdirSync, statSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const version = process.argv[2];
if (!version) {
  console.error('Usage: node npm/scripts/set-version.mjs <version>');
  process.exit(1);
}

// Loose check: digits.digits.digits with an optional prerelease/build tail.
// npm accepts the full semver spec; we only guard against obvious mistakes.
if (!/^\d+\.\d+\.\d+([-.+].*)?$/.test(version)) {
  console.error(`set-version: "${version}" does not look like a semver version`);
  process.exit(1);
}

const npmRoot = join(dirname(fileURLToPath(import.meta.url)), '..');
const mainPkg = '@nuwax-ai/nuwax-grok';
const platformPrefix = '@nuwax-ai/nuwax-grok-';

let updated = 0;
for (const entry of readdirSync(npmRoot)) {
  const pkgPath = join(npmRoot, entry, 'package.json');
  let stats;
  try {
    stats = statSync(pkgPath);
  } catch {
    continue; // not a package directory
  }
  if (!stats.isFile()) continue;

  const pkg = JSON.parse(readFileSync(pkgPath, 'utf8'));
  pkg.version = version;

  if (pkg.name === mainPkg && pkg.optionalDependencies) {
    for (const dep of Object.keys(pkg.optionalDependencies)) {
      if (dep.startsWith(platformPrefix)) {
        pkg.optionalDependencies[dep] = version;
      }
    }
  }

  writeFileSync(pkgPath, `${JSON.stringify(pkg, null, 2)}\n`);
  console.log(`set-version: ${entry} -> ${version}`);
  updated += 1;
}

if (updated === 0) {
  console.error('set-version: no package.json files found under npm/');
  process.exit(1);
}

console.log(`set-version: updated ${updated} package(s) to ${version}`);
