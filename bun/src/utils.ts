import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

export namespace utils {
	export const readFile = (day: number | string, puzzle: number | string) => {
		return readFileSync(resolve(__dirname, '../../_inputs', `day${day}-${puzzle}.txt`)).toString('utf-8')
	}
}
