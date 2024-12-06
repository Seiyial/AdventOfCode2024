import { argv } from 'bun'
import { readFileSync } from 'fs'

export type Ruleset = Record<string, {
	shouldBeRight: Set<string>,
	shouldBeLeft: Set<string>
}>

export const solve = (input: string) => {
	
	const [rulesRows, dataRows] = input.split('\n\n').map((l) => l.split('\n').map((x) => x.trim()))
	const ruleset = rulesRows.reduce((acc, ruleRow) => {
		const [left, right] = ruleRow.split('|')
		acc[left] ??= { shouldBeRight: new Set(), shouldBeLeft: new Set() }
		acc[right] ??= { shouldBeRight: new Set(), shouldBeLeft: new Set() }
		acc[left]!.shouldBeRight.add(right)
		acc[right]!.shouldBeLeft.add(left)
		return acc
	}, {} as Ruleset)
	return dataRows.reduce((acc, row) => {
		row.split(',').every((item, itemIdx) => {
			row.split(',').every((compItem, compItemIdx) => {
				compItemIdx
			})
		})
		return acc
	}, 0)
}

const input = readFileSync(argv[2], 'utf-8')
console.log('Solution:', solve(input))