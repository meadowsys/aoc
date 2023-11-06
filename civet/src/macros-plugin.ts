import { Plugin } from "rollup";
import { walk } from "estree-walker";
import MagicString from "magic-string";
import {} from "estree-walker";

export const aoc_macros: Plugin<void> = {
	name: "aoc-comptime-macros",
	transform(code, id) {
		let before = "civet/src/bin/";
		let after = ".civet.tsx";
		let file = id.slice(before.length, id.length - after.length);

		let transformed = new MagicString(code);

		walk(this.parse(code) as any, {
			enter(node, parent, prop, index) {
				if (
					node.type !== "CallExpression"
					|| node.callee.type !== "Identifier"
				) return;

				if (node.callee.name === "aoc_get_input") {
					let path = `./input/${file}.txt`;
					let test_path = `./input/${file}.debug.txt`;

					transformed.update((node as any).start, (node as any).end, `
						import("fs")
							.then(fs => Promise.all([
								Promise.resolve(fs.existsSync("${test_path}")),
								import("fs/promises")
							]))
							.then(([test_exists, fs]) => {
								if (test_exists && !process.env.NO_DEBUG_DATA) {
									console.log("WARNING: ${test_path} exists, reading input from there. To disable, set \`NO_DEBUG_DATA=1\`");
									return fs.readFile("${test_path}", "utf8")
										.catch(() => {
											console.log("failed to read debug input at ${test_path}, it seemed to exist but couldn't read?");
											process.exit(1);
										});
								}

								return fs.readFile("${path}", "utf8")
									.catch(() => {
										console.log("failed to get input, neither ${path} nor ${test_path} seem to exist");
										process.exit(1);
									});
							})
					`);
				}
			}
		});

		transformed.prepend(`import { format } from "util";`);

		transformed.append(";main();");
		transformed.append(`
			;
			function _panic(pre, pre_with_args, ...args) {
				if (args.length > 0) {
					console.error(format(pre_with_args, ...args));
				} else {
					console.error(pre);
				}

				process.exit(101);
			}

			function panic(...args) { _panic("program panicked", "program panicked:", ...args) }
			function unreachable(...args) { _panic("entered unreachable code", "entered unreachable code:", ...args) }
			function unwrap(e) {
				if (e === undefined) {
					panic("attempted to unwrap an undefined value");
				}
				if (e === null) {
					panic("attempted to unwrap a null value");
				}

				return e;
			}
		`);

		return {
			code: transformed.toString(),
			map: transformed.generateMap()
		};
	},
};

declare global {
	function aoc_get_input(): Promise<string>;
	function panic(...p: Array<any>): never;
	function unreachable(...p: Array<any>): never;
	function unwrap<T>(e: T | undefined | null): T;
}
