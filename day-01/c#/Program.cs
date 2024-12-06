namespace AOCDay1;

class Program
{
	static void Main()
	{
		var input = File.ReadLines("../input.txt");

		var leftList = new List<int>();
		var rightList = new List<int>();

		foreach (var line in input)
		{
			var sides = line.Split("   ");
			leftList.Add(int.Parse(sides[0]));
			rightList.Add(int.Parse(sides[1]));
		}

		var diff = sumOfDifferences(leftList, rightList);
		var sim = sumOfSimilarities(leftList, rightList);

		Console.WriteLine($"The total sum of the differences is {diff}");
		Console.WriteLine($"The total sum of the similarities is {sim}");
	}

	static int sumOfDifferences(List<int> leftList, List<int> rightList)
	{
		leftList.Sort();
		rightList.Sort();

		var differences = 0;

		for (var i = 0; i < leftList.Count; i++)
		{
			differences += Math.Abs(leftList[i] - rightList[i]);
		}

		return differences;
	}

	static int sumOfSimilarities(IEnumerable<int> leftList, IEnumerable<int> rightList)
	{
		// set of all unique IDs in the leftList
		var leftSet = new HashSet<int>(leftList);

		// map of all unique IDs with the key = the ID itself and the value = the number of times it appeared
		var rightMap = new Dictionary<int, int>();
		foreach (var x in rightList) 
		{
			// docs say if the key isn't present, occurrence is set to the default value.
			// which for int, I assume, is 0
			rightMap.TryGetValue(x, out var occurrences);
			rightMap[x] = occurrences + 1;
		}

		var similarities = 0;

		foreach (var id in leftSet)
		{
			rightMap.TryGetValue(id, out var occurrences);
			similarities += id * occurrences;
		}

		return similarities;
	}
}

