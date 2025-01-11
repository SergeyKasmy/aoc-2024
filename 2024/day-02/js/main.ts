const DEBUG = false;
const debug = DEBUG
	? (...args: unknown[]) => {
		console.log("DEBUG:", ...args);
	}
	: () => {};

const input = Deno.readTextFileSync(Deno.args[0]);
const reports = input.trim().split("\n").map((report) =>
	report.split(" ").map((level) => Number(level))
);

debug("Loaded", reports.length, "reports");

const safeReports = reports.reduce(
	(safeReports, report) => safeReports + Number(isReportSafe(report, false)),
	0,
);
const safeReportsSkipped = reports.reduce(
	(safeReports, report) => safeReports + Number(isReportSafe(report, true)),
	0,
);

console.log("Total number of safe reports is:", safeReports);
console.log(
	"Total number of safe reports with 1 value skipped is:",
	safeReportsSkipped,
);

function isReportSafe(report: number[], canSkip: boolean): boolean {
	debug("Report is:", report);

	if (isSequenceSafe(report)) {
		return true;
	}

	if (!canSkip) {
		return false;
	}

	for (let i = 0; i < report.length; i++) {
		const newReport = report.slice(0, i).concat(report.slice(i + 1));
		if (isSequenceSafe(newReport)) {
			return true;
		}
	}

	return false;
}

function isSequenceSafe(report: number[]): boolean {
	let isIncreasing = null;

	for (let i = 1; i < report.length; i++) {
		const delta = report[i] - report[i - 1];

		if (Math.abs(delta) > 3 || delta === 0) {
			return false;
		}

		if (isIncreasing === null) {
			isIncreasing = delta > 0;
		} else if (isIncreasing && delta < 0) {
			return false;
		} else if (!isIncreasing && delta > 0) {
			return false;
		}
	}

	return true;
}
