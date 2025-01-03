use std::{env, fs};

use itertools::Itertools;

fn main() {
    let reports = fs::read_to_string(
        env::args()
            .nth(1)
            .expect("input.txt file path not provided"),
    )
    .expect("couldn't read input.txt")
    .lines()
    .map(|line| {
        line.split(' ')
            .map(|val| val.parse().unwrap())
            .collect::<Vec<u32>>()
    })
    .collect::<Vec<_>>();

    let safe_reports = reports
        .iter()
        .map(|report| u32::from(is_report_safe(report, false)))
        .sum::<u32>();

    let safe_reports_skipped = reports
        .iter()
        .map(|report| u32::from(is_report_safe(report, true)))
        .sum::<u32>();

    println!("Total number of safe reports is: {safe_reports}");
    println!("Total number of safe reports with 1 value skipped is: {safe_reports_skipped}");
}

fn is_report_safe(report: &[u32], can_skip: bool) -> bool {
    if is_sequence_safe(report.iter().copied()) {
        return true;
    }

    if !can_skip {
        return false;
    }

    for i in 0..report.len() {
        let new_report = report[0..i].iter().chain(report[i + 1..].iter()).copied();
        if is_sequence_safe(new_report) {
            return true;
        }
    }

    false
}

fn is_sequence_safe(sequence: impl Iterator<Item = u32>) -> bool {
    let mut is_increasing = None;

    for (prev, next) in sequence.tuple_windows() {
        let delta = next as i32 - prev as i32;

        if delta.abs() > 3 || delta == 0 {
            return false;
        }

        match is_increasing {
            None => is_increasing = Some(delta > 0),
            Some(is_increasing) if is_increasing && delta < 0 => return false,
            Some(is_increasing) if !is_increasing && delta > 0 => return false,
            _ => (),
        }
    }

    true
}
