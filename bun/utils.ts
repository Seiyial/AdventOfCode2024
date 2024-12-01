
export namespace utils {
	export const toLines = (input: string): string[] => {
		return input.split('\n').filter((n) => Boolean(n));
	}

	export const quickSortNumbers = (arr: number[]): number[] => {
		if (arr.length <= 1) {
			return arr;
		}

		const pivot = arr[0];
		const left = [];
		const right = [];

		for (let i = 1; i < arr.length; i++) {
			if (arr[i] < pivot) {
				left.push(arr[i]);
			} else {
				right.push(arr[i]);
			}
		}

		return quickSortNumbers(left).concat(pivot, quickSortNumbers(right));
	}
}
