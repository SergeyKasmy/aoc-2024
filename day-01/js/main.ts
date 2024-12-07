const input = Deno.readTextFileSync(Deno.args[0]);
const lines = input.split(/\n/);

const leftList = [];
const rightList = [];

for (const line of lines) {
	if (line === "") continue;

	const sides = line.split(/\s+/);
	leftList.push(Number(sides[0]));
	rightList.push(Number(sides[1]));
}

console.log("The total sum of the differences is", sumOfDifferences(leftList, rightList));
console.log("The total sum of the similarities is", sumOfSimilarities(leftList, rightList));

// part 1
function sumOfDifferences(leftList: number[], rightList: number[]): number {
	leftList.sort((a, b) => a - b);
	rightList.sort((a, b) => a - b);

	let differences = 0;

	for (let i = 0; i < leftList.length; i++) {
		differences += Math.abs(leftList[i] - rightList[i]);
	}

	return differences;
}

// part 2
function sumOfSimilarities(leftList: number[], rightList: number[]): number {
	// set of all unique IDs in the leftList
	const leftSet = new Set(leftList);

	// map of all unique IDs with the key = the ID itself and the value = the number of times it appeared
	const rightMap = new Map<number, number>();
	for (const x of rightList) {
		rightMap.set(x, (rightMap.get(x) ?? 0) + 1);
	}

	let similarities = 0;
	for (const id of leftSet) {
		similarities += id * (rightMap.get(id) ?? 0);
	}

	return similarities;
}
