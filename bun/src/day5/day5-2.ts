import { RuleMap } from './lib'

export const fixLine = (vals: string[], ruleMap: RuleMap) => {
	const newLine = [...vals]
	return newLine.sort((a, b) => ruleMap.isValidOrder(a, b) ? -1 : 1)
}

export const solve = (input: string) => {
	const [rulesStr, inputStr] = input.split('\n\n')
	const rules = rulesStr.split('\n')
	const ruleMap = RuleMap.fromRules(rules).ensureHasAllRelations()
	return inputStr.split('\n').reduce((sum, line) => {
		const vals = line.split(',')
		const rowIsOk = vals.every((testerVal, testerIdx) => {
			let testerValPositionOk = true
			for (let i = 0; i < vals.length; i++) {
				const v = vals[i]
				if (i < testerIdx) {
					if (testerValPositionOk)
						testerValPositionOk = ruleMap.isValidOrder(v, testerVal)
				} else if (testerIdx < i) {
					if (testerValPositionOk)
						testerValPositionOk = ruleMap.isValidOrder(testerVal, v)
				}
			}
			return testerValPositionOk
		})
		
		if (!rowIsOk) {
			const fixedLine = fixLine(vals, ruleMap)
			return sum + parseInt(fixedLine[(fixedLine.length - 1) / 2])
		} else {
			return sum
		}
	}, 0)
}