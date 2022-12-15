#[derive(Debug, Clone, Copy)]
struct Vertex {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy)]
struct LineSegment {
    slope: f64,
    constant: f64,
    x_min: f64,
    x_max: f64,
}

impl LineSegment {
    fn from(a: &Vertex, b: &Vertex) -> Self {
        let slope = (b.y - a.y) / (b.x - a.x);
        let constant = a.y - slope * a.x;
        let x_min = a.x.min(b.x);
        let x_max = a.x.max(b.x);

        Self { slope, constant, x_min, x_max }
    }

    fn intersection(&self, other: &Self) -> Option<Vertex> {
        let (mut left, mut right) = (self.clone(), other.clone());

        if left.slope == other.slope {
            return None;
        }
        
        left.slope = left.slope - right.slope;
        right.slope = 0.0;

        right.constant = right.constant - left.constant;
        left.constant = 0.0;

        let x = right.constant / left.slope;
    
        if x < self.x_min || x > self.x_max || x < other.x_min || x > other.x_max {
            return None;
        }

        //let y = self.slope * x + self.constant;
        let y = self.y_from_x(x);

        println!("intersects in ({}, {})", x, y);

        Some(Vertex { x, y })
    }

    fn y_from_x(&self, x: f64) -> f64 {
        self.slope * x + self.constant
    }
}

#[derive(Debug, Clone, Copy)]
struct Square {
    vertices: [Vertex; 4],
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

    fn contains(&self, vertex: Vertex) -> bool {
        let ls = self.line_segments();

        vertex.y > ls[0].y_from_x(vertex.x)
        && vertex.y < ls[1].y_from_x(vertex.x)
        && vertex.y < ls[2].y_from_x(vertex.x)
        && vertex.y > ls[3].y_from_x(vertex.x)
    }

    fn line_segments(&self) -> [LineSegment; 4] {
        [
            LineSegment::from(&self.vertices[0], &self.vertices[1]),
            LineSegment::from(&self.vertices[1], &self.vertices[2]),
            LineSegment::from(&self.vertices[2], &self.vertices[3]),
            LineSegment::from(&self.vertices[3], &self.vertices[0]),
        ]
    }

    fn intersections(&self, other: Square) -> Option<Vec<Vertex>> {
        let mut intersections = vec![];

        for self_ls in self.line_segments() {
            for other_ls in other.line_segments() {
                if let Some(intersection) = self_ls.intersection(&other_ls) {
                    intersections.push(intersection)
                }
            }
        }

        if intersections.is_empty() {
            None
        } else {
            Some(intersections)
        }
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
        let test_coord = Coord { x, y: 2000000};

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

    let domain = Square {
        vertices: [
            Vertex { x: 0.0, y: 0.0 },
            Vertex { x: 0.0, y: 20.0 },
            Vertex { x: 20.0, y: 20.0 },
            Vertex { x: 20.0, y: 0.0 },
        ]
    };

    let mut intersections: Vec<Vertex> = vec![];
    for square in &squares {
        for other_square in &squares {
            if let Some(square_intersections) = square.intersections(*other_square) {
                square_intersections
                    .iter()
                    .for_each(|intersection| {
                        if !squares.iter().any(|square| square.contains(*intersection)) {
                            intersections.push(intersection.clone());
                        }
                    });
            }
        }
    }

    for intersection in &intersections {
        if intersection.x >= 0.0 && intersection.x <= 20.0 && intersection.y >= 0.0 && intersection.y <= 20.0 {
        println!("intersection: {:?}", intersection);
        }
    }

    if domain.contains(Vertex { x: 0.1, y: 0.1 }) {
        println!("boomshakalaka");
    }

    0
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3
    ";

    #[ignore]
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 26);
    }

    //#[ignore]
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 56000011);
    }

    use super::{Vertex, Square};
    #[test]
    fn test_square() {

        let square1 = Square { vertices: [
            Vertex { x: 3.0, y: 0.0 },
            Vertex { x: 4.0, y: 1.0 },
            Vertex { x: 3.0, y: 2.0 },
            Vertex { x: 2.0, y: 1.0 },
        ] };

        println!("{:?}", square1);
        println!("{:?}", square1.line_segments());
        println!("{:?}", square1.contains(Vertex { x: 3.0, y: 1.1 }));

        let square2 = Square { vertices: [
            Vertex { x: 3.0, y: 1.0 },
            Vertex { x: 4.0, y: 2.0 },
            Vertex { x: 3.0, y: 3.0 },
            Vertex { x: 2.0, y: 2.0 },
        ] };

        println!("{:?}", square2);
        println!("{:?}", square2.line_segments());
        println!("{:?}", square2.contains(Vertex { x: 3.0, y: 1.1 }));

        println!("{:?}", square1.intersections(square2));

        assert_eq!(1, 0);
    }
}

