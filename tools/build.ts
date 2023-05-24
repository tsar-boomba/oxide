#!/usr/bin/env -S deno run --allow-run --allow-env --allow-net --allow-read --allow-write
import { ensureDirSync, emptyDirSync } from 'https://deno.land/std@0.127.0/fs/mod.ts';
import { cache } from 'https://deno.land/x/cache@0.2.13/mod.ts';

const TOOLCHAIN_VERSION = 'v0.0.3';
const TOOLCHAIN_NAME = 'miyoomini-toolchain.tar.xz';
const TOOLCHAIN_URL = `https://github.com/shauninman/miyoomini-toolchain-buildroot/releases/download/${TOOLCHAIN_VERSION}/${TOOLCHAIN_NAME}`;

const toolchainTar = await cache(TOOLCHAIN_URL);

let numFiles = 0;
try {
	for (const file of Deno.readDirSync('./miyoomini-toolchain')) {
		if (file.isFile) {
			numFiles++;
		}
	}
} catch (_) {
	// If it fails, it doesn't exist
}

// Only decompress if no artifacts exist
if (numFiles <= 0) {
	console.log('No toolchain files found, decompressing archive...');
	new Deno.Command('tar', {
		args: ['xf', toolchainTar.path, '-C', './'],
		stdout: 'piped',
		stderr: 'piped',
	}).outputSync();
} else {
	console.log('Toolchain files found, skipping decompression. 😁');
}

// Build rust stuff
console.log('Begin building rust binary.');
const { code: buildCode, success: buildSuccess } = new Deno.Command('cross', {
	args: ['build', '--release'],
	stdout: 'inherit',
	stderr: 'inherit',
}).outputSync();

if (!buildSuccess) {
	Deno.exit(buildCode);
}

console.log('Creating payload...');
emptyDirSync('build');
ensureDirSync('build/PAYLOAD/miyoo/app/lib');
ensureDirSync('build/PAYLOAD/miyoo/app/bin');

const deps = new Deno.Command('./tools/deps.sh', {
	stderr: 'piped',
	stdout: 'piped',
}).outputSync();

if (!deps.success) {
	console.error(deps.stderr.toLocaleString());
	Deno.exit(deps.code);
}

new Deno.Command('./tools/copy.sh', {
	stdout: 'inherit',
	stderr: 'inherit',
}).outputSync();


