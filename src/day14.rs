const ROWS: usize = 180;
const COLS: usize = 350;
const OFFS: usize = 325;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy,Clone,PartialEq)]
enum Object {
    Rock,
    Sand,
    Air,
}

#[derive(Copy,Clone,Eq,PartialEq)]
struct Coord {
    x: usize,
    y: usize 
}

struct Cave {
    grid: [[Object; COLS]; ROWS]
}

impl Cave {
    fn from(input: &str) -> Result<Self, ()> {
        let mut grid = [[Object::Air; COLS]; ROWS];

        input
            .trim()
            .lines()
            .for_each(|line| {
                line
                    .split(" -> ")
                    .map(|coord| {
                        coord
                            .split_once(',')
                            .map_or(Err(()), |coord| {
                                Ok(Coord {
                                    x: coord.0.parse::<usize>().unwrap() - OFFS,
                                    y: coord.1.parse::<usize>().unwrap(),
                                })
                            })
                            .unwrap()
                    })
                    .into_iter()
                    .collect::<Vec<_>>()
                    .windows(2)
                    .for_each(|line_segment| {
                        let (mut from, to) = (
                            line_segment[0],
                            line_segment[1]
                        );

                        let direction = if from.x < to.x {
                            Direction::Right
                        } else if from.x > to.x {
                            Direction::Left
                        } else if from.y < to.y {
                            Direction::Down
                        } else {
                            Direction::Up
                        };

                        grid[from.y][from.x] = Object::Rock;

                        while from != to {
                            match &direction {
                                Direction::Up => from.y -= 1,
                                Direction::Down => from.y += 1,
                                Direction::Left => from.x -= 1,
                                Direction::Right => from.x += 1
                            }

                            grid[from.y][from.x] = Object::Rock;
                        }
                    });
            });

        Ok(Self { grid })
    }

    fn add_floor(&mut self) {
        let mut floor_y = 0;

        for (y, row) in self.grid.iter().enumerate().rev() {
            if row.contains(&Object::Rock) {
                floor_y = y + 2;
                break;
            }
        }

        self.grid[floor_y] = [Object::Rock; COLS];
    }

    fn fill_with_sand(&mut self) -> usize {
        let mut grains = 0;

        loop {
            let mut grain = Coord { x: 500 - OFFS, y: 0 };

            loop {
                if grain.y == self.grid.len() - 1 {
                    break;
                }

                if self.grid[grain.y + 1][grain.x] == Object::Air {
                    grain.y += 1;
                } else if self.grid[grain.y + 1][grain.x - 1] == Object::Air {
                    grain.y += 1; 
                    grain.x -= 1; 
                } else if self.grid[grain.y + 1][grain.x + 1] == Object::Air {
                    grain.y += 1; 
                    grain.x += 1; 
                } else {
                    break;
                }
            }

            // sand has reached abyss
            if grain.y == self.grid.len() - 1 {
                break;
            }

            self.grid[grain.y][grain.x] = Object::Sand;
            grains += 1;

            // cave is full
            if grain.y == 0 { 
                break;
            }

        }

        grains
    }
}

pub fn part1(input: &str) -> usize {
    let mut cave = Cave::from(input).unwrap();

    cave.fill_with_sand()
}

pub fn part2(input: &str) -> usize {
    let mut cave = Cave::from(input).unwrap();
    cave.add_floor();

    cave.fill_with_sand()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6\n\
                              503,4 -> 502,4 -> 502,9 -> 494,9\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 24);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 93);
    }
}

