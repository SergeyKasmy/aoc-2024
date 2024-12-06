const input = Deno.readTextFileSync("../input.txt");
const lines = input.split(/\n/);

const leftList = [];
const rightList = [];

console.log("Lines:", lines.length);

for (const line of lines) {
	if (line === "") continue;

	const sides = line.split(/\s+/);
	leftList.push(Number(sides[0]));
	rightList.push(Number(sides[1]));
}

leftList.sort((a, b) => a - b);
rightList.sort((a, b) => a - b);

let differences = 0;

for (let i = 0; i < leftList.length; i++) {
	differences += Math.abs(leftList[i] - rightList[i]);
}

console.log("The total sum of the differences is", differences);
