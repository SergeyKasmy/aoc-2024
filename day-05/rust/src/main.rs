use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

type Page = u32;

fn main() {
    let input = fs::read_to_string(
        env::args()
            .nth(1)
            .expect("input.txt file path not provided"),
    )
    .expect("couldn't read input.txt");

    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = parse_rules(rules);
    let updates = parse_updates(updates);

    let midpoint_sum = updates
        .iter()
        .map(|upd| midpoint_if_correctly_ordered(upd, &rules).unwrap_or(0))
        .sum::<u32>();

    println!("Sum of midpoints of all properly ordered updates is {midpoint_sum}");
}

fn parse_rules(rules: &str) -> HashMap<Page, HashSet<Page>> {
    rules.lines().fold(HashMap::new(), |mut acc, line| {
        let (page, before) = line.split_once('|').unwrap();
        let page = page.parse().unwrap();

        acc.entry(page)
            .and_modify(|before_pages| {
                before_pages.insert(before.parse().unwrap());
            })
            .or_default();

        acc
    })
}

fn parse_updates(updates: &str) -> Vec<Vec<Page>> {
    updates
        .lines()
        .map(|line| line.split(',').map(|upd| upd.parse().unwrap()).collect())
        .collect()
}

fn midpoint_if_correctly_ordered(
    update: &[Page],
    rules: &HashMap<Page, HashSet<Page>>,
) -> Option<Page> {
    // check correct order in reverse
    for (idx, page) in update.iter().enumerate().rev() {
        let Some(rule) = rules.get(page) else {
            continue;
        };

        for page_must_be_after in rule {
            // a page that must be after the current one is before
            if update[..idx].contains(page_must_be_after) {
                return None;
            }
        }
    }

    Some(update[update.len() / 2])
}
