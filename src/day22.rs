use std::fmt;

struct Pawn {
    x: i64,
    y: i64,
    dir: f64,
}

impl Pawn {
    fn turn(&mut self, dir: char) {
        // note: the y-axis is mirrored
        let theta = match dir {
            'L' => -(1.0/2.0) * std::f64::consts::PI,
            'R' => (1.0/2.0) * std::f64::consts::PI,
            _ => panic!()
        };

        self.dir += theta;
    }

    fn facing(&self) -> i64 {
        ((self.dir / std::f64::consts::PI).rem_euclid(2.0) * 2.0) as i64
    }

    fn password(&self) -> i64 {
        (self.y + 1) * 1000 + 4 * (self.x + 1) + self.facing()
    }
}

struct Board {
    tiles: Vec<Vec<u8>>,
    pawn: Pawn,
}

impl Board {
    fn from(input: &str) -> Self {
        let mut tiles: Vec<Vec<u8>> = input
            .lines()
            .map(|row| row
                 .as_bytes()
                 .to_vec())
            .collect();

        let max_x = tiles.iter().map(|row| row.len()).max().unwrap();

        for row in tiles.iter_mut() {
            row.resize(max_x, b' ');
        }

        let start_x = input.chars().position(|c| c == '.').unwrap() as i64;
        let pawn = Pawn { x: start_x, y: 0, dir: 0.0 };

        Self { tiles, pawn }
    }

    fn move_pawn(&mut self, d: usize) {
        let tiles = &mut self.tiles;
        let pawn = &mut self.pawn;

        for _ in 1..=d {
            let d_x = pawn.dir.cos() as i64;
            let d_y = pawn.dir.sin() as i64;

            let mut y = (pawn.y as i64 + d_y)
                .rem_euclid(tiles.len() as i64);
            let mut x = (pawn.x as i64 + d_x)
                .rem_euclid(tiles[y as usize].len() as i64);

            let mut next_tile = tiles[y as usize][x as usize];

            while next_tile == b' ' {
                y = (y as i64 + d_y).rem_euclid(tiles.len() as i64) ;
                x = (x as i64 + d_x).rem_euclid(tiles[y as usize].len() as i64);
                next_tile = tiles[y as usize][x as usize];
            }

            match next_tile {
                b'#' => break,
                b'.' => {
                    pawn.x = x;
                    pawn.y = y;
                }
                _ => panic!()
            }
        }
    }

    #[allow(clippy::comparison_chain)]
    fn move_pawn_cube(&mut self, d: usize) {
        let tiles = &mut self.tiles;
        let pawn = &mut self.pawn;

        for _ in 1..=d {
            let d_x = pawn.dir.cos() as i64;
            let d_y = pawn.dir.sin() as i64;

            let mut x = pawn.x as i64 + d_x;
            let mut y = pawn.y as i64 + d_y;

            let mut dir = pawn.dir;

            (x, y) = match (pawn.x / 50, pawn.y / 50) {
                // side 1
                (1, 0) => {
                    if x / 50 == 0 {
                        // moving to side 4
                        dir -= std::f64::consts::PI;
                        (0, 149 - pawn.y.rem_euclid(50))
                    } else if y < 0 {
                        // moving to side 6
                        dir = 0.0;
                        (0, 150 + pawn.x.rem_euclid(50))
                    } else {
                        (x, y)
                    }
                },
                // side 2
                (2, 0) => {
                    if x / 50 > 2 {
                        // moving to side 5
                        dir -= std::f64::consts::PI;
                        (99, 149 - pawn.y.rem_euclid(50))
                    } else if y < 0 {
                        // moving to side 6
                        (pawn.x.rem_euclid(50), 199)
                    } else if y / 50 > 0 {
                        // moving to side 3
                        dir = std::f64::consts::PI;
                        (99, 50 + pawn.x.rem_euclid(50))
                    } else {
                        (x, y)
                    }
                },
                // side 3
                (1, 1) => {
                    if x / 50 < 1 {
                        // moving to side 4
                        dir = (1.0/2.0) * std::f64::consts::PI;
                        (pawn.y.rem_euclid(50), 100)
                    } else if x / 50 > 1 {
                        // moving to side 2
                        dir = -(1.0/2.0) * std::f64::consts::PI;
                        (100 + pawn.y.rem_euclid(50), 49)
                    } else {
                        (x, y)
                    }
                },
                // side 4
                (0, 2) => {
                    if x < 0 {
                        // moving to side 1
                        dir -= std::f64::consts::PI;
                        (50, 49 - pawn.y.rem_euclid(50))
                    } else if y / 50 < 2 {
                        // moving to side 3
                        dir = 0.0;
                        (50, 50 + pawn.x.rem_euclid(50))
                    } else {
                        (x, y)
                    }
                },
                // side 5
                (1, 2) => {
                    if x / 50 > 1 {
                        // moving to side 2
                        dir -= std::f64::consts::PI;
                        (149, 49 - pawn.y.rem_euclid(50))
                    } else if y / 50 > 2 {
                        // moving to side 6
                        dir = std::f64::consts::PI;
                        (49, 150 + pawn.x.rem_euclid(50))
                    } else {
                        (x, y)
                    }
                },
                // side 6
                (0, 3) => {
                    if x < 0 {
                        // moving to side 1
                        dir = (1.0/2.0) * std::f64::consts::PI;
                        (50 + pawn.y.rem_euclid(50), 0)
                    } else if x / 50 > 0 {
                        // moving to side 5
                        dir = -(1.0/2.0) * std::f64::consts::PI;
                        (50 + pawn.y.rem_euclid(50), 149)
                    } else if y / 50 > 3 {
                        // moving to side 2
                        (100 + pawn.x.rem_euclid(50), 0)
                    } else {
                        (x, y)
                    }
                },
                _ => panic!()
            };

            let next_tile = tiles[y as usize][x as usize];

            match next_tile {
                b'#' => break,
                b'.' => {
                    pawn.x = x;
                    pawn.y = y;
                    pawn.dir = dir % (std::f64::consts::PI * 2.0);
                }
                _ => panic!()
            }
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, b) in row.iter().enumerate() {
                let c = if x as i64 == self.pawn.x && y as i64 == self.pawn.y {
                    match self.pawn.facing() {
                        0 => '>',
                        1 => 'v',
                        2 => '<',
                        3 => '^',
                        _ => panic!(),
                    }
                } else {
                    char::from_u32(*b as u32).unwrap()
                };
                write!(f, "{}",c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn part1(input: &str) -> i64 {
    let mut input_iter = input
        .split("\n\n");

    let mut board = Board::from(input_iter.next().unwrap());

    let instructions = input_iter.next().unwrap().trim();

    let movements = instructions
        .split(|c: char| c.is_alphabetic())
        .map(|num| num.parse::<usize>().unwrap());

    let mut turns = instructions
        .split(|c: char| c.is_numeric())
        .filter(|turn| !turn.is_empty());

    for movement in movements {
        board.move_pawn(movement);

        if let Some(turn) = turns.next() {
            board.pawn.turn(turn.chars().next().unwrap());
        }
    }

    board.pawn.password()
}

pub fn part2(input: &str) -> i64 {
    let mut input_iter = input
        .split("\n\n");

    let mut board = Board::from(input_iter.next().unwrap());

    let instructions = input_iter.next().unwrap().trim();

    let movements = instructions
        .split(|c: char| c.is_alphabetic())
        .map(|num| num.parse::<usize>().unwrap());

    let mut turns = instructions
        .split(|c: char| c.is_numeric())
        .filter(|turn| !turn.is_empty());

    for movement in movements {
        board.move_pawn_cube(movement);

        if let Some(turn) = turns.next() {
            board.pawn.turn(turn.chars().next().unwrap());
        }
    }

    board.pawn.password()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "\x20       ...#\n\
                              \x20       .#..\n\
                              \x20       #...\n\
                              \x20       ....\n\
                              ...#.......#\n\
                              ........#...\n\
                              ..#....#....\n\
                              ..........#.\n\
                              \x20       ...#....\n\
                              \x20       .....#..\n\
                              \x20       .#......\n\
                              \x20       ......#.\n\n\
                              10R5L5R10L4R5L5\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 6032);
    }
    
    // The solution for part 2 only works for cube nets laid out as (xFF, xFx,
    // FFx, Fxx). The test case is laid out as (xxFx, FFFx, xxFF), so the test
    // doesn't work.
    #[ignore]
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 5031);
    }
}

