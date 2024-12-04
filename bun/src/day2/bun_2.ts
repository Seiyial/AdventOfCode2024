import { argv } from "bun";
import { readFileSync } from "fs";

export const solve = (input: string) => {
	const aoa: number[][] = input.split("\n").map((row) =>
		row.split(/\s+/).map((num) => parseInt(num, 10))
	);

	const ans = aoa.reduce((acc, line, lineIndex) => {
		console.log("\n\nROW", lineIndex + 1);
		let deltaType: null | 1 | -1 = null;
		const anyAttemptPass = Array.from(
			{ length: line.length + 1 },
			(_, i) => i - 1,
		).some((attemptIdxAndOmitIdx) => {
			console.log(
				attemptIdxAndOmitIdx === -1
					? `\nRow ${lineIndex + 1}`
					: `\nRow ${lineIndex + 1} omit #${attemptIdxAndOmitIdx + 1}`,
			);
			const attemptOk = line
				.filter((v, i) => i !== attemptIdxAndOmitIdx)
				.every((current, cellIndex, arr) => {
					console.log(`#${cellIndex + 1}`, "=", current);
					if (cellIndex === 0) {
						return true;
					} else if (cellIndex === 1) {
						const delta = current - arr[cellIndex - 1];
						console.log(`     (${delta > 0 ? `+${delta}` : delta})`);
						if (delta === 0) {
							return false;
						} else if (Math.abs(delta) > 3) {
							return false;
						} else if (delta > 0) {
							deltaType = 1;
							return true;
						} else if (delta < 0) {
							deltaType = -1;
							return true;
						}
					} else {
						const delta = current - arr[cellIndex - 1];
						console.log(`     (${delta > 0 ? `+${delta}` : delta})`);
						if (delta === 0) return false;
						else if (deltaType === null) {
							throw new Error("didn't set deltaType properly");
						} else if (Math.abs(delta) > 3) return false;
						else if (delta > 0 && deltaType < 0) return false;
						else if (delta < 0 && deltaType > 0) return false;
						else return true;
					}
				});

			console.log(
				attemptOk ? "Passed" : "Failed",
				attemptIdxAndOmitIdx === -1
					? "as-is"
					: `omitting #${attemptIdxAndOmitIdx + 1}`,
			);
			return attemptOk;
		});
		return anyAttemptPass ? acc + 1 : acc;
	}, 0);

	return ans;
};

const input = readFileSync(argv[2], "utf-8");
console.log("Solution:", solve(input));
