use std::{env, fs};

const DEBUG: bool = false;

enum ValidMulOp {
    Yes { result: i32, ends_at: usize },
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

    let mut from = 0;
    let mut sum = 0;

    loop {
        let slice = &input[from..];
        debug!("Len: {}. Sum: {sum}", slice.len());

        match is_valid_mul_op(slice) {
            ValidMulOp::Yes { result, ends_at } => {
                sum += result;
                from += ends_at + 1;

                debug!("Found a valid mul. New from: {from}");
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

    println!("The sum of all valid mul ops is {sum}");
}

fn is_valid_mul_op(s: &str) -> ValidMulOp {
    debug!("Checking {s}");

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

    ValidMulOp::Yes {
        result: first_num * second_num,
        ends_at: paren_idx,
    }
}
