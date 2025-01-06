use std::{env, fs};

const DEBUG: bool = false;

enum ValidMulOp {
    Yes {
        result: i32,
        ends_at: usize,
    },
    /// Skip until this index. Used for ops other than mul or disabled muls
    Skip {
        until: usize,
    },
    No,
}

macro_rules! debug {
    ($($tokens:tt)*) => {
        if DEBUG {
            eprintln!($($tokens)*);
        }
    };
}

fn main() {
    let input = fs::read_to_string(
        env::args()
            .nth(1)
            .expect("input.txt file path not provided"),
    )
    .expect("couldn't read input.txt");

    let full_sum = find_all_ops::<false>(&input);
    let enabled_sum = find_all_ops::<true>(&input);

    println!("The sum of all mul ops is {full_sum}");
    println!("The sum of all enabled mul ops is {enabled_sum}");
}

fn find_all_ops<const PROCESS_DO_DONTS: bool>(input: &str) -> i32 {
    let mut from = 0;
    let mut sum = 0;
    let mut enabled = true;

    loop {
        let slice = &input[from..];
        debug!("Len: {}. Sum: {sum}", slice.len());

        match is_valid_mul_op::<PROCESS_DO_DONTS>(slice, &mut enabled) {
            ValidMulOp::Yes { result, ends_at } => {
                sum += result;
                from += ends_at + 1;

                debug!("Found a valid mul. New from: {from}");
            }
            ValidMulOp::Skip { until } => {
                from += until + 1;

                debug!("Found a disabled mul or other op. New from: {from}");
            }
            ValidMulOp::No => {
                from += 1;

                debug!("Not found. New from: {from}");
            }
        }

        if from >= input.len() {
            debug!("Reached the end of input");
            break;
        }
    }

    sum
}

fn is_valid_mul_op<const PROCESS_DO_DONTS: bool>(s: &str, enabled: &mut bool) -> ValidMulOp {
    debug!("Checking {s}");

    if PROCESS_DO_DONTS {
        if s.starts_with("do()") {
            *enabled = true;
            return ValidMulOp::Skip { until: 3 };
        } else if s.starts_with("don't()") {
            *enabled = false;
            return ValidMulOp::Skip { until: 6 };
        }
    }

    if s.len() < 4 {
        return ValidMulOp::No;
    }

    if &s[..3] != "mul" {
        return ValidMulOp::No;
    }

    if &s[3..4] != "(" {
        return ValidMulOp::No;
    }

    // s now starts after the first paren
    let s = &s[4..];

    let Some(comma_idx) = s.find(',') else {
        return ValidMulOp::No;
    };

    let Ok(first_num) = s[..comma_idx].parse::<i32>() else {
        return ValidMulOp::No;
    };

    let paren_idx = match s[comma_idx + 1..].find(')') {
        Some(paren_idx) => paren_idx + comma_idx + 1, // search started at comma_idx + 1, add it back
        _ => return ValidMulOp::No,
    };

    let Ok(second_num) = s[comma_idx + 1..paren_idx].parse::<i32>() else {
        return ValidMulOp::No;
    };

    debug!("First num: {first_num}, second num: {second_num}");

    if PROCESS_DO_DONTS {
        if !*enabled {
            return ValidMulOp::Skip { until: paren_idx };
        }
    }

    ValidMulOp::Yes {
        result: first_num * second_num,
        ends_at: paren_idx,
    }
}
