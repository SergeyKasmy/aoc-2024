const input = Deno.readTextFileSync(Deno.args[0]);
const reports = input.trim().split("\n").map((report) => report.split(" ").map((level) => Number(level)));
const safeReports = reports.reduce((safeReports, report) => safeReports + Number(isReportSafe(report)), 0);

console.log("Total number of safe reports is:", safeReports);

function isReportSafe(report: number[]): boolean {
	// console.log("Report is:", report);

	const isIncreasing = report[0] - report[1] < 0;
	// console.log("isIncreasing:", isIncreasing);

	for (let i = 1; i < report.length; i++) {
		const delta = report[i - 1] - report[i];
		// console.log("Delta of", report[i - 1], "and", report[i], "is", delta);

		if (Math.abs(delta) > 3) {
			// console.log("delta too big, not safe");
			return false;
		}

		if (isIncreasing) {
			// if delta is descreasing
			if (delta >= 0) {
				// console.log("delta is descreasing after initially increasing, not safe");
				return false;
			}
		} else {
			if (delta <= 0) {
				// console.log("delta is inscreasing after initially descreasing, not safe");
				return false;
			}
		}
	}

	// console.log("All checks passed, it's safe:", report);
	return true;
}
