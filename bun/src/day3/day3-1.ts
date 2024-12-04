import { utils } from '../utils'

const matchables = /(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don\'t\(\))/g;
export const solve = (input: string) => {
	return input
		.matchAll(matchables)
		?.reduce((acc, [currMatch]) => acc +
			currMatch.slice(4, -1).split(',').reduce((a, b) => a * +b, 1)
		, 0)
}

console.log("Solution:", solve(utils.readFile(3, 'a')));
