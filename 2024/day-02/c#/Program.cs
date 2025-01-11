namespace AOCDay1;

class Program
{
	static void Main(string[] args)
	{
		var input = File.ReadLines(args[0]).Select(line => {
			return line.Split(' ').Select(val => int.Parse(val)).ToArray();
		}).ToArray();


		var safeReports = input.Select(report => Convert.ToInt32(isReportSafe(report, false))).Sum();
		var safeReportsSkipped = input.Select(report => Convert.ToInt32(isReportSafe(report, true))).Sum();

		Console.WriteLine($"Total number of safe reports is: {safeReports}");
		Console.WriteLine($"Total number of safe reports with 1 value skipped is: {safeReportsSkipped}");
	}

	static bool isReportSafe(int[] report, bool canSkip) {
		if (isSequenceSafe(report)) {
			return true;
		}

		if (!canSkip) {
			return false;
		}

		for (int i = 0; i < report.Length; i++) {
			var new_report = report.Take(i).Concat(report.Skip(i + 1)).ToArray();

			if (isSequenceSafe(new_report)) {
				return true;
			}
		}

		return false;
	}

	static bool isSequenceSafe(int[] report) {
		bool? isIncreasing = null;

		for (int i = 1; i < report.Length; i++) {
			var delta = report[i] - report[i - 1];

			if (Math.Abs(delta) > 3 || delta == 0) {
				return false;
			}

			if (isIncreasing == null) {
				isIncreasing = delta > 0;
			} else if (isIncreasing.Value && delta < 0) {
				return false;
			} else if (!isIncreasing.Value && delta > 0) {
				return false;
			}
		}

		return true;
	}
}

