use std::{collections::HashSet, env, fmt::Display, fs};

const GRID_SIZE: usize = 130;

#[derive(Clone)]
struct Map {
    grid: Grid,
    guard: Guard,
    history: History,
}

#[derive(Clone)]
struct Grid([[Element; GRID_SIZE]; GRID_SIZE]);

#[derive(Clone, Copy)]
enum Element {
    Air,
    Path,
    Obsticle,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Guard {
    x: isize,
    y: isize,
    facing: Facing,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
enum History {
    No,
    Yes(HashSet<Guard>),
}

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
    map.history = History::Yes(HashSet::new());

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let mut map = map.clone();
            let target_elem = &mut map.grid.0[i][j];

            let Element::Air = target_elem else {
                continue;
            };

            if map.guard.x as usize == i && map.guard.y as usize == j {
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
                            x: row as isize,
                            y: column as isize,
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
            history: History::No,
        }
    }

    /// Returns true if the guard went off the map
    fn step(&mut self) -> bool {
        let (next_x, next_y) = self.guard.next();

        let Some(next) = self.grid.get(next_x, next_y) else {
            return true;
        };

        match next {
            Element::Air | Element::Path => {
                self.guard.move_forwards();
                self.grid.set_path(next_x, next_y);
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
        if let History::Yes(history) = &mut self.history {
            return history.insert(self.guard.clone());
        }

        true
    }
}

impl Grid {
    fn get(&self, x: isize, y: isize) -> Option<Element> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;

        self.0.get(x)?.get(y).copied()
    }

    fn set_path(&mut self, x: isize, y: isize) {
        let x: usize = x.try_into().expect("invalid x");
        let y: usize = y.try_into().expect("invalid y");

        self.0[x][y] = Element::Path;
    }
}

impl Guard {
    fn next(&self) -> (isize, isize) {
        let mut new_guard = self.clone();
        new_guard.move_forwards();

        (new_guard.x, new_guard.y)
    }

    fn move_forwards(&mut self) {
        match self.facing {
            Facing::Up => self.x -= 1,
            Facing::Down => self.x += 1,
            Facing::Left => self.y -= 1,
            Facing::Right => self.y += 1,
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

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, line) in self.grid.0.iter().enumerate() {
            for (j, elem) in line.iter().enumerate() {
                let ch = match elem {
                    Element::Air | Element::Path
                        if self.guard.x == i as isize && self.guard.y == j as isize =>
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
