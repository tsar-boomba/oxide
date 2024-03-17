#!/usr/bin/env -S deno run --allow-run --allow-env --allow-net --allow-read --allow-write
import { ensureDirSync, emptyDirSync, copySync } from 'https://deno.land/std@0.192.0/fs/mod.ts';
import { parseArgs } from 'https://deno.land/std@0.207.0/cli/parse_args.ts';
import { loadSync } from 'https://deno.land/std@0.218.0/dotenv/mod.ts';
import { cache } from 'https://deno.land/x/cache@0.2.13/mod.ts';
import { getCores } from './cores.ts';

const TOOLCHAIN_VERSION = 'v0.0.3';
const TOOLCHAIN_NAME = 'miyoomini-toolchain.tar.xz';
const TOOLCHAIN_URL = `https://github.com/shauninman/miyoomini-toolchain-buildroot/releases/download/${TOOLCHAIN_VERSION}/${TOOLCHAIN_NAME}`;

const buildEnv = loadSync({
	envPath: './.env.build',
	examplePath: './env.build.example',
	allowEmptyValues: true,
});

// Parse CLI arguments
const {
	native,
	release,
	verbose,
	output,
	'build-only': buildOnly,
	['--']: extraCargoArgs,
} = parseArgs(Deno.args, {
	boolean: ['native', 'release', 'verbose', 'build-only'],
	string: ['output'],
	alias: { native: 'N', verbose: 'v', 'build-only': 'B' },
	default: { native: false, verbose: false, 'build-only': false, output: '/Volumes/OXIDE-DEV' },
	'--': true,
});

// Build rust stuff
console.log('Begin building rust binary.');

const cargoArgs: string[] = [];

if (release) {
	cargoArgs.push('--release');
}

let build: Deno.CommandOutput;
if (native) {
	// Just use cargo
	console.log('Using native cross-compilation.');

	if (Object.keys(buildEnv).length !== 0 && verbose) {
		console.log('Build environment:', buildEnv);
	}

	if (verbose) {
		console.log('Cargo Args: ', [...cargoArgs, ...extraCargoArgs]);
	}

	build = new Deno.Command('cargo', {
		args: [
			'build',
			'--target',
			'armv7-unknown-linux-gnueabihf',
			...cargoArgs,
			...extraCargoArgs,
		],
		stdout: 'inherit',
		stderr: 'inherit',
		stdin: 'null',
	}).outputSync();
} else {
	if (Deno.build.arch === 'aarch64') {
		// Have separate build script for arm hosts to avoid QEMU slowdown
		console.log('Using ARM Docker image to build.');
		build = new Deno.Command('./tools/arm-build.sh', {
			stdout: 'inherit',
			stderr: 'inherit',
			stdin: 'null',
		}).outputSync();
	} else {
		console.log('Using cross to build.');
		build = new Deno.Command('cross', {
			args: ['build', ...cargoArgs],
			env: {
				DOCKER_BUILDKIT: '1',
				CROSS_CONTAINER_OPTS: '--platform linux/amd64',
			},
			stdout: 'inherit',
			stderr: 'inherit',
			stdin: 'null',
		}).outputSync();
	}
}

if (!build.success) {
	Deno.exit(build.code);
}

if (buildOnly) {
	console.log('Skipping payload generation.');
	Deno.exit(0);
}

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

emptyDirSync('build/bin');
emptyDirSync('build/PAYLOAD');
ensureDirSync('build/lib');
ensureDirSync('build/bin');

console.log('Getting cores...');
await getCores();

console.log('Building weston...');
const weston = new Deno.Command('./tools/weston.sh', {
	stderr: 'piped',
	stdout: 'piped',
}).outputSync();

if (!weston.success) {
	console.error(new TextDecoder().decode(weston.stdout));
	Deno.exit(weston.code);
}

console.log('Creating payload...');
const copy = new Deno.Command('./tools/copy.sh', {
	args: [release ? 'release' : 'debug'],
	stdout: 'inherit',
	stderr: 'inherit',
}).outputSync();

if (!copy.success) {
	Deno.exit(copy.code);
}

// TODO: make this work on any plat and support any sd card name
console.log(`Adding files to ${output}...`);
new Deno.Command('rm', { args: ['-rf', `${output}/miyoo/app`] }).outputSync();
copySync('build/PAYLOAD/', output, { overwrite: true });

if (output.startsWith('/Volumes') && Deno.build.os === 'darwin') {
	// If on mac and writing to an sd card, eject it afterwards

	console.log('Ejecting SD card...');
	const eject = new Deno.Command('diskutil', {
		args: ['eject', output],
		stdout: 'inherit',
		stderr: 'inherit',
	}).outputSync();

	if (!eject.success) {
		console.error(`Failed to eject ${output}`);
		Deno.exit(eject.code);
	}
}

console.log(`Finished build.`);
