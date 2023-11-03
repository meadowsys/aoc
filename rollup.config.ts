import { defineConfig } from "rollup";
import { globSync } from "glob";
import path from "path";
import civet from "@danielx/civet/rollup";
import typescript from "rollup-plugin-typescript2"

const civet_src = "civet/src/bin";
const civet_build = `target/civet`;

export default defineConfig({
	input: Object.fromEntries(
		globSync("civet/src/bin/y*-d*.civet", { }).map(f => [
			path.relative(civet_src, f.slice(0, f.length - path.extname(f).length)),
			path.resolve(f)
		] as const)
	),
	output: {
		format: "esm",
		dir: civet_build,
		entryFileNames: "[name].mjs"
	},
	plugins: [
		typescript(),
		civet({})
	]
});
