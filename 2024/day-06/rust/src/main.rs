use aoc::{Grid, Point};
use std::{env, fmt::Display, fs};

const GRID_SIZE: usize = 130;

#[derive(Clone)]
struct Map {
    grid: Grid<Element, GRID_SIZE>,
    guard: Guard,
    history: Option<History>,
}

#[derive(Clone, Copy)]
enum Element {
    Air,
    Path,
    Obsticle,
}

#[derive(Clone, Debug)]
struct Guard {
    pos: Point<isize>,
    facing: Facing,
}

#[derive(Clone, Copy, Debug)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
// 4 = the number of facing values that can exist
struct History([[[bool; 4]; GRID_SIZE]; GRID_SIZE]);

#[derive(PartialEq, Eq, Debug)]
enum WalkResult {
    GuardLeftTheMap,
    GuardStuckInALoop,
}

fn main() {
    let input = fs::read_to_string(
        env::args()
            .nth(1)
            .expect("input.txt file path not provided"),
    )
    .expect("couldn't read input.txt");

    let map = Map::parse(&input);

    let path_count = part1(map.clone());
    let loop_posibilities = part2(map);

    println!("Total path count: {path_count}");
    println!("Loop posibilities: {loop_posibilities}");
}

fn part1(mut map: Map) -> usize {
    let walk_res = map.walk();
    assert_eq!(walk_res, WalkResult::GuardLeftTheMap);

    map.grid
        .0
        .iter()
        .flatten()
        .filter(|elem| matches!(elem, Element::Path))
        .count()
}

fn part2(mut map: Map) -> usize {
    let mut looped = 0;
    map.history = Some(History::default());

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let mut map = map.clone();
            let target_elem = &mut map.grid.0[i][j];

            let Element::Air = target_elem else {
                continue;
            };

            if map.guard.pos.x as usize == i && map.guard.pos.y as usize == j {
                // don't add obsticles at the current guard pos
                // assume we can't loop the guard in the default map config
                continue;
            }

            *target_elem = Element::Obsticle;

            if let WalkResult::GuardStuckInALoop = map.walk() {
                looped += 1;
            }
        }
    }

    looped
}

impl Map {
    fn parse(input: &str) -> Map {
        let mut grid = [[Element::Air; 130]; 130];
        let mut guard = None;

        for (row, line) in input.lines().enumerate() {
            for (column, ch) in line.char_indices() {
                match ch {
                    '.' => grid[row][column] = Element::Air,
                    '#' => grid[row][column] = Element::Obsticle,
                    '^' => {
                        grid[row][column] = Element::Air;
                        guard = Some(Guard {
                            pos: Point { x: row, y: column }.into(),
                            facing: Facing::Up,
                        });
                    }
                    _ => panic!("Unknown element: {ch}"),
                }
            }
        }

        Map {
            grid: Grid(grid),
            guard: guard.expect("guard not found"),
            history: None,
        }
    }

    /// Returns true if the guard went off the map
    fn step(&mut self) -> bool {
        let Some(next_pos) = self.guard.next().as_usize() else {
            return true;
        };

        let Some(next) = self.grid.get(next_pos) else {
            return true;
        };

        match next {
            Element::Air | Element::Path => {
                self.guard.move_forwards();
                self.grid[next_pos] = Element::Path;
            }
            Element::Obsticle => self.guard.turn_right(),
        }

        false
    }

    fn walk(&mut self) -> WalkResult {
        self.save_current_pos();

        while !self.step() {
            if !self.save_current_pos() {
                return WalkResult::GuardStuckInALoop;
            }
        }

        WalkResult::GuardLeftTheMap
    }

    /// Returns true if the guard has never been in this position
    fn save_current_pos(&mut self) -> bool {
        if let Some(history) = &mut self.history {
            return history.insert(&self.guard);
        }

        true
    }
}

impl Guard {
    fn next(&self) -> Point<isize> {
        let mut new_guard = self.clone();
        new_guard.move_forwards();
        new_guard.pos
    }

    fn move_forwards(&mut self) {
        match self.facing {
            Facing::Up => self.pos.x -= 1,
            Facing::Down => self.pos.x += 1,
            Facing::Left => self.pos.y -= 1,
            Facing::Right => self.pos.y += 1,
        }
    }

    fn turn_right(&mut self) {
        self.facing = match self.facing {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
        }
    }
}

impl History {
    // Returns true if the guard has never been in this position
    fn insert(&mut self, guard: &Guard) -> bool {
        let pos = guard
            .pos
            .as_usize()
            .expect("An out of range guard pos can't be saved");
        assert!(pos.x < GRID_SIZE, "Guard pos x bigger than grid size");
        assert!(pos.y < GRID_SIZE, "Guard pos y bigger than grid size");

        let cell = &mut self.0[pos.x][pos.y][guard.facing as usize];

        let is_already_present = *cell;
        *cell = true;

        !is_already_present
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, line) in self.grid.0.iter().enumerate() {
            for (j, elem) in line.iter().enumerate() {
                let ch = match elem {
                    Element::Air | Element::Path
                        if self.guard.pos.x == i as isize && self.guard.pos.y == j as isize =>
                    {
                        '*'
                    }
                    Element::Air => '.',
                    Element::Path => 'X',
                    Element::Obsticle => '#',
                };

                write!(f, "{ch}")?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl Default for History {
    fn default() -> Self {
        Self([[[false; 4]; GRID_SIZE]; GRID_SIZE])
    }
}
