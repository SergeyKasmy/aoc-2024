use std::{cmp::Ordering, env, fs};

type Page = usize;

fn main() {
    let input = fs::read_to_string(
        env::args()
            .nth(1)
            .expect("input.txt file path not provided"),
    )
    .expect("couldn't read input.txt");

    let (rules, updates) = input.split_once("\n\n").unwrap();

    let order = parse_rules(rules);
    let mut updates = parse_updates(updates);

    let mut ordered_midpoint_sum = 0;
    let mut unordered_midpoint_sum = 0;

    for update in &mut updates {
        let midpoint = midpoints(update, &order);

        if midpoint.is_sorted {
            ordered_midpoint_sum += midpoint.value;
        } else {
            unordered_midpoint_sum += midpoint.value;
        }
    }

    println!("Sum of midpoints of all properly ordered updates is {ordered_midpoint_sum}");
    println!("Sum of midpoints of all manually ordered updates is {unordered_midpoint_sum}");
}

fn parse_rules(rules: &str) -> [[Ordering; 100]; 100] {
    rules
        .lines()
        .fold([[Ordering::Greater; 100]; 100], |mut acc, line| {
            let (page, must_be_after) = line.split_once('|').unwrap();
            let page: Page = page.parse().unwrap();
            let must_be_after: Page = must_be_after.parse().unwrap();

            acc[page][must_be_after] = Ordering::Less;

            acc
        })
}

fn parse_updates(updates: &str) -> Vec<Vec<Page>> {
    updates
        .lines()
        .map(|line| line.split(',').map(|upd| upd.parse().unwrap()).collect())
        .collect()
}

struct Midpoint {
    value: Page,
    is_sorted: bool,
}

fn midpoints(update: &mut [Page], order: &[[Ordering; 100]; 100]) -> Midpoint {
    let is_sorted = update.is_sorted_by(|&a, &b| order[a][b] == Ordering::Less);

    let midpoint = if is_sorted {
        update[update.len() / 2]
    } else {
        let (_, &mut midpoint, _) =
            update.select_nth_unstable_by(update.len() / 2, |&a, &b| order[a][b]);
        midpoint
    };

    Midpoint {
        value: midpoint,
        is_sorted,
    }
}
