import { argv } from 'bun'
import { readFileSync } from 'fs'

const fwdPattern = /XMAS/
const revPattern = /SAMX/

type EqLongSlices = string[]
const _makeVerticals = (horiz: EqLongSlices): EqLongSlices => {
	return horiz[0].split('').map((first, vertIdx) => (
		horiz.map((row) => row[vertIdx]).join('')
	))
}
const _makeDiagonalsUp = (horiz: EqLongSlices): string[] => {

	// traverse L shape starting point
	// min length is 4
	// start going vertically down
	const diagsUp: string[] = []
	let startingRowIdx = 0
	// downward traverse
	while (startingRowIdx < horiz.length) {
		let rowIdx = startingRowIdx
		let colIdx = 0
		const diag: string[] = []
		while (rowIdx >= 0) {
			diag.push(horiz[rowIdx][colIdx])
			colIdx += 1
			rowIdx -= 1
		}
		// just include all short diags for now
		diagsUp.push(diag.join(''))
		startingRowIdx += 1
	}
	
	// now bottom left to bottom right, and i is startingCol
	// skip 0 because the last case above handled it
	const colLen = horiz[0].length
	let startingColIdx = 1
	while (startingColIdx < colLen) {
		const diag: string[] = []
		let rowIdx = horiz.length - 1
		let colIdx = startingColIdx
		while (colIdx < colLen) {
			diag.push(horiz[rowIdx][colIdx])
			rowIdx -= 1
			colIdx += 1
		}
		diagsUp.push(diag.join(''))
		startingColIdx += 1
	}

	return diagsUp
}

const _makeDiagonalsDown = (horiz: EqLongSlices): string[] => {

	// traverse from bottom left to top left, then top left to top right
	// start going vertically up
	const diagsDown: string[] = []
	let startingRowIdx = horiz.length - 1
	// downward traverse
	while (startingRowIdx >= 0) {
		let rowIdx = startingRowIdx
		let colIdx = 0
		const diag: string[] = []
		while (rowIdx < horiz.length) {
			diag.push(horiz[rowIdx][colIdx])
			colIdx += 1
			rowIdx += 1
		}
		// just include all short diags for now
		diagsDown.push(diag.join(''))
		startingRowIdx -= 1
	}
	
	// now bottom left to bottom right, and i is startingCol
	// skip 0 because the last case above handled it
	const colLen = horiz[0].length
	let startingColIdx = 1
	while (startingColIdx < colLen) {
		const diag: string[] = []
		let rowIdx = 0
		let colIdx = startingColIdx
		while (colIdx < colLen) {
			diag.push(horiz[rowIdx][colIdx])
			rowIdx += 1
			colIdx += 1
		}
		diagsDown.push(diag.join(''))
		startingColIdx += 1
	}

	return diagsDown
}

const ensureEqualLengthStrings = (input: string[]) => (
	input.every((row) => row.length === row[0].length) || (new Error('array not equal length strings'))
)

export const solve = (input: string) => {
	
	const horizontals: EqLongSlices = input.split('\n')
	ensureEqualLengthStrings(horizontals)
	const verticals: EqLongSlices = _makeVerticals(horizontals)
	ensureEqualLengthStrings(horizontals)
	const diagonalsUp: EqLongSlices = _makeDiagonalsUp(horizontals)
	const diagonalsDown: EqLongSlices = _makeDiagonalsDown(horizontals)

	const lists = [...horizontals, ...verticals, ...diagonalsUp, ...diagonalsDown]

	return [...lists.join(' ').matchAll(/XMAS/g), ...lists.join(' ').matchAll(/SAMX/g)].length
	
}

const input = readFileSync(argv[2], 'utf-8')
console.log('Solution:', solve(input))