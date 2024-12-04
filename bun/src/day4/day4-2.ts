import { argv } from 'bun'
import { readFileSync } from 'fs'

const SAM_OR_MAS = new Set<string>(['SAM', 'MAS'])

export const solve = (input: string) => {
	type Coord = [x: number, y: number]
	type CoordList = Coord[]
	const horiz: string[] = input.split('\n')
	// get all coords of A's
	return horiz.reduce((coordList, row, rowIdx) => (
		coordList.concat(row.split('').reduce((coordList, cell, colIdx) => (
			cell === 'A' && coordList.push([rowIdx, colIdx]),
			coordList
		), <CoordList>[]))
	), <CoordList>[])
	.reduce((acc, [x, y]) => {
		const diagDown = `${horiz[x-1]?.[y-1]}A${horiz[x+1]?.[y+1]}`
		const diagUp = `${horiz[x-1]?.[y+1]}A${horiz[x+1]?.[y-1]}`
		return acc + ((SAM_OR_MAS.has(diagDown) && SAM_OR_MAS.has(diagUp)) ? 1 : 0)
	}, 0)
}

const input = readFileSync(argv[2], 'utf-8')
console.log('Solution:', solve(input))