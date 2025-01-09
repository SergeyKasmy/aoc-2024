use std::{env, fs};

use directions::DIRECTIONS;

type Direction = [(isize, isize); 3];

#[rustfmt::skip]
mod directions {
    use crate::Direction;

    const RIGHT: Direction = [(0, 1), (0, 2), (0, 3)];
    const LEFT: Direction  = [(0, -1), (0, -2), (0, -3)];
    const UP: Direction    = [(-1, 0), (-2, 0), (-3, 0)];
    const DOWN: Direction  = [(1, 0), (2, 0), (3, 0)];

    const fn merge(a: Direction, b: Direction) -> Direction {
        [
            (a[0].0 + b[0].0, a[0].1 + b[0].1),
            (a[1].0 + b[1].0, a[1].1 + b[1].1),
            (a[2].0 + b[2].0, a[2].1 + b[2].1),
        ]
    }

    pub const DIRECTIONS: &[Direction] = &[
        RIGHT,
        LEFT,
        UP,
        DOWN,
        merge(RIGHT, UP),
        merge(RIGHT, DOWN),
        merge(LEFT, UP),
        merge(LEFT, DOWN),
    ];
}

fn main() {
    let input = fs::read_to_string(
        env::args()
            .nth(1)
            .expect("input.txt file path not provided"),
    )
    .expect("couldn't read input.txt")
    .lines()
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

    let mut xmas_num = 0;

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] != 'X' {
                continue;
            }

            xmas_num += xmas_num_from_current_pos(&input, (i, j));
        }
    }

    println!("XMAS'es found: {xmas_num}");
}

/// Returns the number of XMAS'es that could be reached from the current position in the grid
fn xmas_num_from_current_pos(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> usize {
    assert!(grid[pos.0][pos.1] == 'X');

    // number of xmases originating from this X
    let mut xmas_num = 0;

    for direction in DIRECTIONS {
        if get_by_offset(grid, pos, direction[0]).is_some_and(|c| c == 'M')
            && get_by_offset(grid, pos, direction[1]).is_some_and(|c| c == 'A')
            && get_by_offset(grid, pos, direction[2]).is_some_and(|c| c == 'S')
        {
            xmas_num += 1
        }
    }

    xmas_num
}

/// Returns the value of the element by the offset from a position
fn get_by_offset(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
    offset: (isize, isize),
) -> Option<char> {
    grid.get(offset_from_index(pos.0, offset.0))?
        .get(offset_from_index(pos.1, offset.1))
        .copied()
}

/// Calculates a new index by an offset
fn offset_from_index(index: usize, offset: isize) -> usize {
    (index as isize + offset) as usize
}
