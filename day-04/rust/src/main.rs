use std::{env, fs};

use directions::{ALL_DIRECTIONS, DIAG_DIRECTIONS};

type Grid = Vec<Vec<char>>;
type Direction<const N: usize> = [(isize, isize); N];

#[rustfmt::skip]
mod directions {
    use crate::Direction;

    // starting from 0, 0
    const RIGHT: Direction<4> = [(0, 0), (0, 1), (0, 2), (0, 3)];
    const LEFT: Direction<4>  = [(0, 0), (0, -1), (0, -2), (0, -3)];
    const UP: Direction<4>    = [(0, 0), (-1, 0), (-2, 0), (-3, 0)];
    const DOWN: Direction<4>  = [(0, 0), (1, 0), (2, 0), (3, 0)];

    // starting from the corner
    const RIGHT_DOWN: Direction<3> = [(-1, -1), (0, 0), (1, 1)];
    const RIGHT_UP: Direction<3>   = [(1, -1), (0, 0), (-1, 1)];

    const fn merge(a: Direction<4>, b: Direction<4>) -> Direction<4> {
        [
            (a[0].0 + b[0].0, a[0].1 + b[0].1),
            (a[1].0 + b[1].0, a[1].1 + b[1].1),
            (a[2].0 + b[2].0, a[2].1 + b[2].1),
            (a[3].0 + b[3].0, a[3].1 + b[3].1),
        ]
    }

    const fn reverse(dir: Direction<3>) -> Direction<3> {
        [
            (dir[2].0, dir[2].1),
            (dir[1].0, dir[1].1),
            (dir[0].0, dir[0].1),
        ]
    }

    pub const ALL_DIRECTIONS: &[Direction<4>] = &[
        RIGHT,
        LEFT,
        UP,
        DOWN,
        merge(RIGHT, UP),
        merge(RIGHT, DOWN),
        merge(LEFT, UP),
        merge(LEFT, DOWN),
    ];

    pub const DIAG_DIRECTIONS: &[Direction<3>] = &[
        RIGHT_UP,
        RIGHT_DOWN,
        reverse(RIGHT_UP),
        reverse(RIGHT_DOWN),
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

    let xmas_num = find_word_in_grid::<4, false>(&input, ['X', 'M', 'A', 'S'], ALL_DIRECTIONS);
    let mas_num = find_word_in_grid::<3, true>(&input, ['M', 'A', 'S'], DIAG_DIRECTIONS);

    println!("XMAS'es found: {xmas_num}");
    println!("MAS'es in an X found: {mas_num}");
}

fn find_word_in_grid<const N: usize, const COUNT_2_FOR_1: bool>(
    grid: &Grid,
    word: [char; N],
    directions: &[Direction<N>],
) -> usize {
    let mut word_num = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let count = word_count_from_pos(&grid, word, directions, (i, j));

            word_num += if COUNT_2_FOR_1 {
                if count == 2 {
                    1
                } else {
                    0
                }
            } else {
                count
            };
        }
    }

    word_num
}

/// Returns the number of the word that could be reached in the specified directions from the current position in the grid
fn word_count_from_pos<const N: usize>(
    grid: &Grid,
    word: [char; N],
    directions: &[Direction<N>],
    pos: (usize, usize),
) -> usize {
    // number of words originating from this position
    let mut word_num = 0;

    'direction: for direction in directions {
        for (idx, letter) in word.iter().copied().enumerate() {
            if get_by_offset(grid, pos, direction[idx]).is_none_or(|c| c != letter) {
                continue 'direction;
            }
        }

        word_num += 1;
    }

    word_num
}

/// Returns the value of the element by the offset from a position
fn get_by_offset(grid: &Grid, pos: (usize, usize), offset: (isize, isize)) -> Option<char> {
    grid.get(offset_from_index(pos.0, offset.0))?
        .get(offset_from_index(pos.1, offset.1))
        .copied()
}

/// Calculates a new index by an offset
fn offset_from_index(index: usize, offset: isize) -> usize {
    (index as isize + offset) as usize
}
