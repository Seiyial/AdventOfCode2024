import { argv } from "bun";
import { readFileSync } from "node:fs";
import { utils } from "../../utils";

const _sep = "   ";
export const solve = (input: string): number => {
	const { left, right } = utils
		.toLines(input)
		.reduce((acc, line) => {
			const [first, second] = line.split(_sep).map((n) => Number(n));
			console.log({ first, second });
			acc.left.push(first);
			acc.right.push(second);
			return acc;
		}, { left: <number[]> [], right: <number[]> [] });

	const leftSorted = utils.quickSortNumbers(left);
	const rightSorted = utils.quickSortNumbers(right);
	// if (leftSorted.length !== rightSorted.length) {
	// 	throw new Error('Invalid input')
	// }

	let count = 0;
	for (let i = 0; i < leftSorted.length; i++) {
		const diff = Math.abs(leftSorted[i] - rightSorted[i]);
		count += diff;
	}

	return count;
};

console.log("Solution:", solve(readFileSync(argv[2], "utf-8")));
