use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../input.txt");

fn main() {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = INPUT
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
        })
        .unzip();

    println!(
        "The total sum of the differences is {}",
        sum_of_differences(&mut left, &mut right)
    );

    println!(
        "The total sum of the similarities is {}",
        sum_of_similarities(&left, &right)
    );
}

fn sum_of_differences(left: &mut [i32], right: &mut [i32]) -> i32 {
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .fold(0, |acc, (&left, &right)| acc + (left - right).abs())
}

fn sum_of_similarities(left: &[i32], right: &[i32]) -> i32 {
    // set of all unique IDs in the left list
    let left_set = left.iter().copied().collect::<HashSet<_>>();

    // map of all unique IDs with the key = the ID itself and the value = the number of times it appeared
    let right_map = right.iter().fold(
        HashMap::<i32, i32>::new(),
        |mut right_map_by_occurance, &id| {
            *right_map_by_occurance.entry(id).or_default() += 1;
            right_map_by_occurance
        },
    );

    left_set.into_iter().fold(0, |acc, id| {
        acc + id * right_map.get(&id).copied().unwrap_or_default()
    })
}
