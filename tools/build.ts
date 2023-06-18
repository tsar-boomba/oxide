#!/usr/bin/env -S deno run --allow-run --allow-env --allow-net --allow-read --allow-write
import { ensureDirSync, emptyDirSync } from 'https://deno.land/std@0.127.0/fs/mod.ts';
import { cache } from 'https://deno.land/x/cache@0.2.13/mod.ts';

const TOOLCHAIN_VERSION = 'v0.0.3';
const TOOLCHAIN_NAME = 'miyoomini-toolchain.tar.xz';
const TOOLCHAIN_URL = `https://github.com/shauninman/miyoomini-toolchain-buildroot/releases/download/${TOOLCHAIN_VERSION}/${TOOLCHAIN_NAME}`;

console.log('Getting toolchain...');
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
let build: Deno.CommandOutput;
if (Deno.build.arch === 'aarch64') {
	build = new Deno.Command('./tools/arm-build.sh', {
		stdout: 'inherit',
		stderr: 'inherit',
	}).outputSync();
} else {
	build = new Deno.Command('cross', {
		args: ['build', '--release'],
		env: {
			DOCKER_BUILDKIT: '1',
			CROSS_CONTAINER_OPTS: '--platform linux/amd64',
		},
		stdout: 'inherit',
		stderr: 'inherit',
	}).outputSync();
}

if (!build.success) {
	Deno.exit(build.code);
}

emptyDirSync('build');
ensureDirSync('build/PAYLOAD/miyoo/app/lib');
ensureDirSync('build/PAYLOAD/miyoo/app/bin');

console.log('Getting deps...');
const deps = new Deno.Command('./tools/deps.sh', {
	stderr: 'piped',
	stdout: 'piped',
}).outputSync();

if (!deps.success) {
	console.error(deps.stderr.toLocaleString());
	Deno.exit(deps.code);
}

console.log('Building weston...');
const weston = new Deno.Command('./tools/weston.sh', {
	stderr: 'piped',
	stdout: 'piped',
}).outputSync();

if (!weston.success) {
	console.error(weston.stderr.toLocaleString());
	Deno.exit(weston.code);
}

console.log('Creating payload...');
new Deno.Command('./tools/copy.sh', {
	stdout: 'inherit',
	stderr: 'inherit',
}).outputSync();
