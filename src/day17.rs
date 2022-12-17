#![allow(clippy::int_plus_one)]
#![allow(clippy::needless_range_loop)]

#[derive(Clone,Copy,PartialEq)]
enum Shape {
    Minus,
    Plus,
    MirrorL,
    Pipe,
    Square
}

#[derive(Clone,Copy,PartialEq)]
struct Rock {
    shape: Shape,
    x: i64,
    y: i64
}

impl Rock {
    fn width(&self) -> usize {
        match self.shape {
            Shape::Minus => 4,
            Shape::Plus => 3,
            Shape::MirrorL => 3,
            Shape::Pipe => 1,
            Shape::Square => 2
        }
    }

    fn height(&self) -> usize {
        match self.shape {
            Shape::Minus => 1,
            Shape::Plus => 3,
            Shape::MirrorL => 3,
            Shape::Pipe => 4,
            Shape::Square => 2
        }
    }
}

struct RockMaker {
    prev: Option<Shape>
}

impl RockMaker {
    fn new() -> Self {
        Self { prev: None } 
    }

    fn make(&mut self, x: i64, y: i64) -> Rock {
        let shape = match self.prev {
            None => Shape::Minus,
            Some(Shape::Minus) => Shape::Plus,
            Some(Shape::Plus) => Shape::MirrorL,
            Some(Shape::MirrorL) => Shape::Pipe,
            Some(Shape::Pipe) => Shape::Square,
            Some(Shape::Square) => Shape::Minus
        };

        self.prev = Some(shape);

        Rock { shape, x, y }
    }
}

#[derive(Clone)]
struct Cave {
    rows: Vec<[bool; 7]>
}

impl Cave {
    fn height(&self) -> usize {
        self.rows.len()
    }

    fn collides_with_wall(&self, rock: &Rock) -> bool {
        rock.x < 0 || rock.x + rock.width() as i64 - 1 > 6 || rock.y < 0
    }

    fn collides(&self, rock: &Rock) -> bool {
        // check for collisions with cave wall
        if self.collides_with_wall(rock) {
            return true;
        }

        let (rock_x, rock_y) = (rock.x as usize, rock.y as usize);
        let rows = &self.rows;
        let height = self.height();

        // return false if new rock is higher than any existing rock
        if rock_y >= height {

            return false;
        }

        match rock.shape {
            Shape::Minus => {
                for x in rock_x..(rock_x + rock.width()) {
                    if rows[rock_y][x] {
                        return true;
                    }
                }
            },
            Shape::Plus => {
                // check rock's bottom row against grid
                if rows[rock_y][rock_x + 1] {
                    return true;
                }

                // check rock's middle row against grid
                if height > rock_y + 1 {
                    for x in rock_x..(rock_x + rock.width()) {
                        if rows[rock_y + 1][x] {
                            return true;
                        }
                    }
                }

                // check rock's top row against grid
                if height > rock_y + 2 && rows[rock_y + 2][rock_x + 1] {
                    return true;
                }
            },
            Shape::MirrorL => {
                // check rock's bottom row against grid
                for x in rock_x..(rock_x + rock.width()) {
                    if rows[rock_y][x] {
                        return true;
                    }
                }

                // check rock's middle row against grid
                if height > rock_y + 1 && rows[rock_y + 1][rock_x + 2] {
                    return true;
                }

                // check rock's top row against grid
                if height  > rock_y + 2 && rows[rock_y + 2][rock_x + 2] {
                    return true;
                }
            },
            Shape::Pipe => {
                // check each row against rock_x
                for y in rock_y..height.min(rock_y + rock.height()) {
                    if rows[y][rock_x] {
                        return true;
                    }
                }
            },
            Shape::Square => {
                // check rock's bottom row against grid
                for x in rock_x..(rock_x + rock.width()) {
                    if rows[rock_y][x] {
                        return true;
                    }
                }

                // check rock's middle row against grid
                if height - 1 >= rock_y + 1 {
                    for x in rock_x..(rock_x + rock.width()) {
                        if rows[rock_y + 1][x] {
                            return true;
                        }
                    }
                }
            },
        }

        let mut test_cave = self.clone();
        test_cave.add(rock);

        false
    }

    fn add(&mut self, rock: &Rock) {
        let cave_height = self.height();
        let rows = &mut self.rows;

        let height_diff = rock.y + rock.height() as i64 - cave_height as i64;
        for _ in 0..height_diff {
            rows.push([false; 7]);
        }

        let (rock_x, rock_y) = (rock.x as usize, rock.y as usize);
        
        match rock.shape {
            Shape::Minus => {
                rows[rock_y][rock_x..rock_x + rock.width()].fill(true);
            },
            Shape::Plus => {
                rows[rock_y][rock_x + 1] = true;
                rows[rock_y + 1][rock_x..rock_x + rock.width()].fill(true);
                rows[rock_y + 2][rock_x + 1] = true;
            },
            Shape::MirrorL => {
                rows[rock_y][rock_x..rock_x + rock.width()].fill(true);
                rows[rock_y + 1][rock_x + 2] = true;
                rows[rock_y + 2][rock_x + 2] = true;
            },
            Shape::Pipe => {
                for y in rock_y..(rock_y + rock.height()) {
                    rows[y][rock_x] = true;
                }
            },
            Shape::Square => {
                for y in rock_y..(rock_y + rock.height()) {
                    rows[y][rock_x..rock_x + rock.width()].fill(true);
                }
            },
        }
    }
}

pub fn part1(input: &str) -> usize {
    let jets_input = input.trim().as_bytes().iter();
    let mut jets = jets_input.clone();
    let mut rockmaker = RockMaker { prev: None };

    let mut cave = Cave { rows: vec![] };


    for _ in 0..2022 {
        let mut rock = rockmaker.make(2, cave.height() as i64 + 3);

        loop {
            if jets.len() == 0 {
                jets = jets_input.clone();
            }

            let mut moved_rock = rock;

            moved_rock.x = match jets.next().unwrap() {
                b'<' => moved_rock.x - 1,
                b'>' => moved_rock.x + 1,
                _ => panic!()
            };

            if cave.collides(&moved_rock) {
                moved_rock = rock;
            }

            moved_rock.y -= 1;

            // if rock can't drop any further, add rock to cave
            if cave.collides(&moved_rock) {
                moved_rock.y += 1;
                cave.add(&moved_rock);
                break;
            }

            rock = moved_rock;
        }
    }

    cave.height()
}

struct Groundhog {
    rock: Rock,
    start_i: usize,
    start_height: usize,
    end_i: Option<usize>,
    end_height: Option<usize>
}

impl Groundhog {
    fn delta_i(&self) -> Result<usize, ()> {
        if let Some(end_i) = self.end_i {
            Ok(end_i - self.start_i)
        } else {
            Err(())
        }
    }

    fn delta_height(&self) -> Result<usize, ()> {
        if let Some(end_height) = self.end_height {
            Ok(end_height - self.start_height)
        } else {
            Err(())
        }
    }
}

pub fn part2(input: &str) -> usize {
    let jets_input = input.trim().as_bytes().iter();
    let mut jets = jets_input.clone();

    let mut rockmaker = RockMaker { prev: None };
    let mut cave = Cave { rows: vec![] };
    let mut groundhog: Option<Groundhog> = None;

    let mut i = 0;

    while groundhog.is_none() || groundhog.as_ref().unwrap().end_i.is_none() {
        let mut rock = rockmaker.make(2, cave.height() as i64 + 3);

        loop {
            // check if we have run out of jets
            if jets.len() == 0 {

                if let Some(hog) = &mut groundhog {
                    // if we have already found a groundhog, check if we can use
                    // the current rock as an offset and store the amount of
                    // iterations it took to get here and the difference in 
                    // stack height
                    if hog.rock.shape == rock.shape && hog.rock.x == rock.x {
                        hog.rock = rock;
                        hog.end_i = Some(i);
                        hog.end_height = Some(cave.height());
                        break;
                    } else if hog.rock.shape == rock.shape {
                        // something has gone wrong, the shape matches but
                        // the x coordinate doesn't. never mind, take this
                        // rock as groundhog instead
                        groundhog = Some(Groundhog { 
                            rock,
                            start_i: i,
                            start_height: cave.height(),
                            end_i: None,
                            end_height: None
                        });
                    }
                } else {
                    // if we hadn't yet picked a groundhog, use this one 
                    groundhog = Some(Groundhog {
                        rock,
                        start_i: i,
                        start_height: cave.height(),
                        end_i: None,
                        end_height: None
                    });
                }

                // get a new iterator over our jets
                jets = jets_input.clone();
            }

            let mut moved_rock = rock;

            moved_rock.x = match jets.next().unwrap() {
                b'<' => moved_rock.x - 1,
                b'>' => moved_rock.x + 1,
                _ => panic!()
            };

            if cave.collides(&moved_rock) {
                moved_rock = rock;
            }

            moved_rock.y -= 1;

            if cave.collides(&moved_rock) {
                moved_rock.y += 1;
                cave.add(&moved_rock);
                break;
            }

            rock = moved_rock;
        }

        i += 1;
    }

    let groundhog = groundhog.unwrap();

    let count = (1000000000000usize - i) / groundhog.delta_i().unwrap();
    let skipped_height = count * groundhog.delta_height().unwrap();

    i += count * groundhog.delta_i().unwrap() - 1;

    let mut groundhog = Some(groundhog);

    while i < 1000000000000usize {
        let mut rock = if let Some(hog) = groundhog {
            hog.rock
        } else {
            rockmaker.make(2, cave.height() as i64 + 3)
        };
        groundhog = None;

        loop {
            if jets.len() == 0 {
                jets = jets_input.clone();
            }

            let mut moved_rock = rock;

            moved_rock.x = match jets.next().unwrap() {
                b'<' => moved_rock.x - 1,
                b'>' => moved_rock.x + 1,
                _ => panic!()
            };

            if cave.collides(&moved_rock) {
                moved_rock = rock;
            }

            moved_rock.y -= 1;

            if cave.collides(&moved_rock) {
                moved_rock.y += 1;
                cave.add(&moved_rock);
                break;
            }

            rock = moved_rock;
        }

        i += 1;
    }

    cave.height() + skipped_height
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3068);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1514285714288);
    }
}

