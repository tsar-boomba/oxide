#!/usr/bin/env -S deno run --allow-run --allow-env --allow-net --allow-read --allow-write
/**
 * Goes through the sysroot /usr/lib/arm-linux-gnueabihf dir and removes symlinks to /lib/arm-linux-gnueabihf
 */

console.log('Fixing symlinks libs...');

for (const entry of Deno.readDirSync('./build/sysroot/usr/lib/arm-linux-gnueabihf')) {
	if (entry.isSymlink) {
		// We needa fix it
		const filePath = './build/sysroot/usr/lib/arm-linux-gnueabihf/' + entry.name;
		const linkDestination = Deno.readLinkSync(filePath);

		if (linkDestination.startsWith('/lib')) {
			console.log(`${filePath} -> ${linkDestination}`);
			// This link would be broken during copying
			const newLinkDestination = Deno.cwd() + '/build/sysroot' + linkDestination;

			// Remove broken link & replace with fixed link
			Deno.removeSync(filePath);
			new Deno.Command('ln', { args: [newLinkDestination, filePath] }).outputSync();
		}
	}
}

console.log('Done fixing symlinks.');
