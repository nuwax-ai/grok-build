'use strict';

// Each platform sub-package exports the absolute path to its bundled `grok`
// executable. The main launcher (`@nuwax-ai/nuwax-grok/bin.js`) requires this
// module to discover the binary, then spawns it.

const { join } = require('node:path');

const exe = process.platform === 'win32' ? 'grok.exe' : 'grok';
module.exports = join(__dirname, 'bin', exe);
