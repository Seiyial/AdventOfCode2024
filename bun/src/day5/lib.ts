
export class OrderedItem {
	public val: string
	public pre: Set<string>
	public post: Set<string>

	constructor(val: string) {
		this.post = new Set()
		this.pre = new Set()
		this.val = val
	}
}

export class RuleMap {
	public map: Record<string, OrderedItem> = {}

	public static fromRules (rules: string[]) {
		return rules.reduce((acc, ruleStr) => {
			const [left, right] = ruleStr.split('|')
			acc.addRule(left, right)
			return acc
		}, new RuleMap())
	}

	public addRule(left: string, right: string) {
		this.map[left] ??= new OrderedItem(left)
		this.map[left]!.post.add(right)
		this.map[right] ??= new OrderedItem(right)
		this.map[right]!.pre.add(left)
	}

	public isValidOrder(before: string, after: string) {
		return !(this.map[before]?.pre.has(after) ?? false)
		// simpler version is:
		// return this.map[before]?.post.has(after) ?? false
		// but I wanted it to be fault-tolerant to just in case not all 
		// relations are specified in input data
		// so instead of "does the left items's itemsOnTheRight have the right item?"
		// it is "does the left it's itemsOnTheLeft NOT have the right item?".
	}

	public ensureHasAllRelations() {
		const first = Object.values(this.map)[0]
		if (!first) throw Error('rulemap is empty')
		const numRelations = first.pre.size + first.post.size
		if (
			Object.values(this.map).every(
				(item) => item.pre.size + item.post.size === numRelations
			)
		) {
			console.log('Yes, rulemap has all relations')
			return this
		} else {
			throw new Error('Rulemap does not have equal sized relations')
		}
	}
}