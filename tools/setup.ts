#!/usr/bin/env -S deno run --allow-run --allow-env --allow-net --allow-read --allow-write

import { ensureDirSync, existsSync } from 'https://deno.land/std@0.192.0/fs/mod.ts';

console.log('Creating sysroot...');
const sysroot = new Deno.Command('./tools/sysroot.sh', {
	stderr: 'piped',
	stdout: 'piped',
}).outputSync();

if (!sysroot.success) {
	console.error(sysroot.stderr.toLocaleString());
	Deno.exit(sysroot.code);
}

const llvmBin =
	Deno.env.get('LLVM_BIN') ??
	new TextDecoder()
		.decode(new Deno.Command('which', { args: ['llvm-objdump'] }).outputSync().stdout)
		.replace('/llvm-objdump', '')
		.trim();

if (!llvmBin) {
	console.log(
		`Couldn't find LLVM binutils! Make sure llvm/bin is in your PATH or specify with LLVM_BIN environment variable.`
	);
	Deno.exit(1);
}

console.log('Found LLVM bins at:', llvmBin);

const binutils = 'build/llvm-bin';

ensureDirSync(binutils);

if (existsSync(binutils)) {
	Deno.exit(0);
}

Deno.linkSync(llvmBin + '/llvm-ar', binutils + '/ar');
Deno.linkSync(llvmBin + '/lld', binutils + '/ld');
Deno.linkSync(llvmBin + '/llvm-objcopy', binutils + '/objcopy');
Deno.linkSync(llvmBin + '/llvm-objdump', binutils + '/objdump');
Deno.linkSync(llvmBin + '/llvm-ranlib', binutils + '/ranlib');
Deno.linkSync(llvmBin + '/llvm-strings', binutils + '/strings');