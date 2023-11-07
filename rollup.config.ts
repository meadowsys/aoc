import { defineConfig } from "rollup";
import { globSync } from "glob";
import path from "path";
import civet from "@danielx/civet/rollup";
import typescript from "rollup-plugin-typescript2";
import { aoc_macros } from "./civet/src/macros-plugin";

const civet_src = "civet/src/yyyy/";
const civet_build = `target/civet`;

export default defineConfig({
	input: Object.fromEntries(
		globSync("civet/src/*/y*-d*.civet").map(f => [
			f.slice(civet_src.length, f.length - path.extname(f).length),
			path.resolve(f)
		] as const)
	),
	output: {
		format: "esm",
		dir: civet_build,
		entryFileNames: "[name].mjs",
		sourcemap: "inline"
	},
	plugins: [
		typescript(),
		civet({}),
		aoc_macros
	],
	external: [
		"fs",
		"fs/promises",
		"util"
	]
});
