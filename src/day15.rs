#[derive(Debug, Clone, Copy)]
struct Vertex {
    x: f64,
    y: f64,
}

impl Vertex {
    fn rotate(&mut self, theta: f64) {
        let x = self.x * theta.cos() - self.y * theta.sin();
        let y = self.x * theta.sin() + self.y * theta.cos();

        self.x = x;
        self.y = y;
    }
}

#[derive(Debug)]
struct Square {
    vertices: [Vertex; 4]
}

impl Square {
    fn from(sensor: &Sensor) -> Self {
        Self {
            vertices: [
                Vertex { 
                    x: sensor.pos.x as f64,
                    y: sensor.pos.y as f64 - sensor.beacon_distance() as f64,
                },
                Vertex {
                    x: sensor.pos.x as f64 + sensor.beacon_distance() as f64 + 1.0,
                    y: sensor.pos.y as f64
                },
                Vertex { 
                    x: sensor.pos.x as f64,
                    y: sensor.pos.y as f64 + sensor.beacon_distance() as f64 + 1.0,
                },
                Vertex {
                    x: sensor.pos.x as f64 - sensor.beacon_distance() as f64,
                    y: sensor.pos.y as f64
                },
            ]
        }
    }

    fn right(&self) -> (Vertex, Vertex) {
        (self.vertices[0], self.vertices[1])
    }

    fn top(&self) -> (Vertex, Vertex) {
        (self.vertices[1], self.vertices[2])
    }

    fn left(&self) -> (Vertex, Vertex) {
        (self.vertices[2], self.vertices[3])
    }

    fn bottom(&self) -> (Vertex, Vertex) {
        (self.vertices[3], self.vertices[0])
    }

    fn min_x(&self) -> f64 {
        self
            .vertices
            .iter()
            .map(|vertex| vertex.x)
            .reduce(|min, cur| if cur < min { cur } else { min })
            .unwrap()
    }

    fn max_x(&self) -> f64 {
        self
            .vertices
            .iter()
            .map(|vertex| vertex.x)
            .reduce(|max, cur| if cur > max { cur } else { max })
            .unwrap()
    }

    fn min_y(&self) -> f64 {
        self
            .vertices
            .iter()
            .map(|vertex| vertex.y)
            .reduce(|min, cur| if cur < min { cur } else { min })
            .unwrap()

    }

    fn max_y(&self) -> f64 {
        self
            .vertices
            .iter()
            .map(|vertex| vertex.y)
            .reduce(|max, cur| if cur > max { cur } else { max })
            .unwrap()
    }

    fn contains(&self, vertex: Vertex) -> bool {
        // allows for floating point errors
        vertex.x > self.min_x() + 1e-10
        && vertex.x < self.max_x() - 1e-10
        && vertex.y > self.min_y() + 1e-10
        && vertex.y < self.max_y() - 1e-10
    }

    fn rotate(&mut self, theta: f64) {
        for i in 0..self.vertices.len() {
            self.vertices[i].rotate(theta);
        }
    }

    fn intersections(&self, other: &Self) -> Vec<Vertex> {
        let mut intersections = vec![];

        // right top
        if self.right().0.x >= other.top().1.x
           && self.right().0.x <= other.top().0.x
           && other.top().0.y >= self.right().0.y
           && other.top().0.y <= self.right().1.y {
            let intersection = Vertex {
                x: self.right().0.x,
                y: other.top().0.y,
            };

            intersections.push(intersection);
        }

        // right bottom
        if self.right().0.x >= other.bottom().0.x
           && self.right().0.x <= other.bottom().1.x
           && other.bottom().0.y >= self.right().0.y
           && other.bottom().0.y <= self.right().1.y {
            let intersection = Vertex {
                x: self.right().0.x,
                y: other.bottom().0.y,
            };

            intersections.push(intersection);
        }

        // top left
        if self.top().0.y <= other.left().0.y
           && self.top().0.y >= other.left().1.y
           && other.left().0.x <= self.top().0.x
           && other.left().0.x >= self.top().1.x {
            let intersection = Vertex {
                x: other.left().0.x,
                y: self.top().0.y,
            };

            intersections.push(intersection);
        }

        // top right
        if self.top().0.y <= other.right().1.y
           && self.top().0.y >= other.right().0.y
           && other.right().0.x <= self.top().0.x
           && other.right().0.x >= self.top().1.x {
            let intersection = Vertex {
                x: other.right().0.x,
                y: self.top().0.y,
            };

            intersections.push(intersection);
        }

        // left top
        if self.left().0.x >= other.top().1.x
           && self.left().0.x <= other.top().0.x
           && other.top().0.y >= self.left().1.y
           && other.top().0.y <= self.left().0.y {
            let intersection = Vertex {
                x: self.left().0.x,
                y: other.top().0.y,
            };

            intersections.push(intersection);
        }

        // left bottom
        if self.left().0.x >= other.bottom().0.x
           && self.left().0.x <= other.bottom().1.x
           && other.bottom().0.y >= self.left().1.y
           && other.bottom().0.y <= self.left().0.y {
            let intersection = Vertex {
                x: self.left().0.x,
                y: other.bottom().0.y,
            };

            intersections.push(intersection);
        }

        // bottom left
        if self.bottom().0.y <= other.left().0.y
           && self.bottom().0.y >= other.left().1.y
           && other.left().0.x <= self.bottom().1.x
           && other.left().0.x >= self.bottom().0.x {
            let intersection = Vertex {
                x: other.left().0.x,
                y: self.bottom().0.y,
            };

            intersections.push(intersection);
        }

        // bottom right
        if self.bottom().0.y <= other.right().1.y
           && self.bottom().0.y >= other.right().0.y
           && other.right().0.x <= self.bottom().1.x
           && other.right().0.x >= self.bottom().0.x {
            let intersection = Vertex {
                x: other.right().0.x,
                y: self.bottom().0.y,
            };

            intersections.push(intersection);
        }

        intersections
    }

}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn manhattan_distance(&self, other: Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug)]
struct Beacon {
    pos: Coord,
}

#[derive(Debug)]
struct Sensor {
    pos: Coord,
    beacon: Beacon,
}

impl Sensor {
    fn from(input: &str) -> Self {
        let mut input = input.split(['=', ',', ':']);

        let pos = Coord {
            x: input.nth(1).unwrap().parse::<i64>().unwrap(),
            y: input.nth(1).unwrap().parse::<i64>().unwrap(),
        };

        let beacon = Beacon { 
            pos: Coord {
                x: input.nth(1).unwrap().parse::<i64>().unwrap(),
                y: input.nth(1).unwrap().parse::<i64>().unwrap(),
            }
        };

        Self { pos, beacon }

    }

    fn beacon_distance(&self) -> u64 {
        self.pos.manhattan_distance(self.beacon.pos)
    }

    fn can_see(&self, coord: Coord) -> bool {
        self.pos.manhattan_distance(coord) <= self.beacon_distance()
    }

}

pub fn part1(input: &str) -> usize {
    let sensors: Vec<Sensor> = input
        .trim()
        .lines()
        .map(Sensor::from)
        .collect();

    let mut occupied: usize = 0;

    for x in -1000000i64..10000000 {
        let test_coord = Coord { x, y: 2000000 };

        for sensor in &sensors {
            if test_coord == sensor.beacon.pos {
                continue;
            }

            if sensor.can_see(test_coord) {
                occupied += 1;
                break;
            }
        }
    }

    occupied
}

pub fn part2(input: &str) -> usize {
    let sensors: Vec<Sensor> = input
        .trim()
        .lines()
        .map(Sensor::from)
        .collect();

    // turn each sensor into a square defind by 
    let mut squares: Vec<Square> = sensors
        .iter()
        .map(Square::from)
        .collect();

    squares.iter().for_each(|square| println!("{:?}", square));

    for square in squares.iter_mut() {
        square.rotate(0.25 * std::f64::consts::PI);
    }

    let mut intersections: Vec<Vertex> = vec![];

    for square in squares.iter() {
        for square2 in squares.iter() {
            intersections.append(&mut square.intersections(square2));
        }
    }

    let domain = Square {
        vertices: [
            Vertex { x: 0.0, y: 0.0 },
            Vertex { x: 4000000.0, y: 0.0 },
            Vertex { x: 4000000.0, y: 4000000.0 },
            Vertex { x: 0.0, y: 4000000.0 },
        ]
    };

    let mut free_intersections: Vec<Vertex> = vec![];

    for intersection in &intersections {
        if squares.iter().all(|square| !square.contains(*intersection)) {
            let mut free_intersection = intersection.clone();
            free_intersection.rotate(-0.25 * std::f64::consts::PI);

            if domain.contains(free_intersection) {
                free_intersections.push(free_intersection);
            }
        }
    }

    for i in 0..free_intersections.len() {
        println!("intersection: {:?}", free_intersections[i]);
    }

    0
}

#[cfg(test)]
mod tests {
use super::{part1, part2};

const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
                          Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
                          Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
                          Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
                          Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
                          Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
                          Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
                          Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
                          Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
                          Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
                          Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
                          Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
                          Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
                          Sensor at x=20, y=1: closest beacon is at x=15, y=3\n";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 26);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 56000011);
}
}

