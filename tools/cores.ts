// Gets cores from OnionOS repo (because im lazy 💀)
import { pooledMap } from 'https://deno.land/std@0.192.0/async/pool.ts';
import { ensureDir } from 'https://deno.land/std@0.192.0/fs/ensure_dir.ts';

const CORES = [
	'gambatte',
	'mgba',
	'vbam',
	'vba_next',
	'gpsp',
	'snes9x',
	'snes9x2010',
	'snes9x2005_plus',
	'snes9x2002',
] as const;

const getCore = async (name: string) => {
	const path = `build/cores/${name}_libretro.so`;
	try {
		// Check if file exists
		const file = await Deno.open(path, { read: true });

		// If we got here, it exists. No need to get it again.

		file.close();
	} catch (e) {
		if (e instanceof Deno.errors.NotFound) {
			// Doesn't exist
			const res = await fetch(
				`https://github.com/OnionUI/Onion/raw/main/static/build/RetroArch/.retroarch/cores/${name}_libretro.so`
			);

			if (!res.ok) throw new Error(`Core ${name} has an error ${res.status}`);

			const stream = res.body;
			if (!stream) throw new Error(`No stream for ${name}`);
			await Deno.writeFile(path, stream, { append: false, create: true });
		} else {
			// Real error 😱
			console.error(`Error opening ${path}:`, e?.toString());
			Deno.exit(1);
		}
	}
};

/** Gets cores and puts them into build/lib */
export const getCores = async () => {
	await ensureDir('build/cores');
	const promises = CORES.map((name) => getCore(name));
	// Use a pooled map to ensure we don't do too many requests at once
	const results = pooledMap(5, promises, (p) => p);

	results.next()

	for await (const _ of results) {
		// Just wait for all cores to be processed
	}
};
