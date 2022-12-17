#[derive(Clone, Copy, PartialEq)]
struct Vertex {
    x: f64,
    y: f64,
}

impl Vertex {
    fn manhattan_distance(&self, other: Self) -> u64 {
        (self.x as i64).abs_diff(other.x as i64) + (self.y as i64).abs_diff(other.y as i64)
    }
}

#[derive(Clone, Copy)]
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

    fn intersect(&self, other: &Self) -> Option<Vertex> {
        let (mut left, mut right) = (*self, *other);

        // segments with same slope have none or many solutions
        if left.slope == other.slope {
            return None;
        }

        left.slope -= right.slope;
        right.slope = 0.0;

        right.constant -= left.constant;
        left.constant = 0.0;

        let x = right.constant / left.slope;
    
        if x < self.x_min || x > self.x_max || x < other.x_min || x > other.x_max {
            return None;
        }

        let y = self.y_from_x(x);

        Some(Vertex { x, y })
    }

    fn y_from_x(&self, x: f64) -> f64 {
        self.slope * x + self.constant
    }
}

#[derive(Clone, Copy)]
struct Square {
    vertices: [Vertex; 4],
}

impl Square {
    fn from(sensor: &Sensor) -> Self {
        Self {
            vertices: [
                Vertex { 
                    x: sensor.pos.x,
                    y: sensor.pos.y - sensor.beacon_distance() as f64 - 1.0,
                },
                Vertex {
                    x: sensor.pos.x + sensor.beacon_distance() as f64 + 1.0,
                    y: sensor.pos.y
                },
                Vertex { 
                    x: sensor.pos.x,
                    y: sensor.pos.y + sensor.beacon_distance() as f64 + 1.0,
                },
                Vertex {
                    x: sensor.pos.x - sensor.beacon_distance() as f64 - 1.0,
                    y: sensor.pos.y
                },
            ]
        }
    }

    fn contains(&self, vertex: Vertex) -> bool {
        let line_segments = self.line_segments();

        if line_segments.iter().any(|segment| segment.slope.is_infinite()) {
            // square is parallel to axes
            vertex.x >= self.vertices[3].x
            && vertex.x <= self.vertices[0].x
            && vertex.y >= self.vertices[0].y
            && vertex.y <= self.vertices[1].y
        } else {
            // square is not parallel to axes
            vertex.y > line_segments[0].y_from_x(vertex.x)
            && vertex.y < line_segments[1].y_from_x(vertex.x)
            && vertex.y < line_segments[2].y_from_x(vertex.x)
            && vertex.y > line_segments[3].y_from_x(vertex.x)
        }
    }

    fn line_segments(&self) -> [LineSegment; 4] {
        [
            LineSegment::from(&self.vertices[0], &self.vertices[1]),
            LineSegment::from(&self.vertices[1], &self.vertices[2]),
            LineSegment::from(&self.vertices[2], &self.vertices[3]),
            LineSegment::from(&self.vertices[3], &self.vertices[0]),
        ]
    }

    fn intersect_line_segment(&self, other: &LineSegment) -> Option<Vec<Vertex>>{
        let mut intersections = vec![];

        for self_ls in self.line_segments() {
            if let Some(intersection) = self_ls.intersect(other) {
                intersections.push(intersection);
            }
        }

        if intersections.is_empty() {
            None
        } else {
            Some(intersections)
        }
    }

    fn intersect_square(&self, other: &Self) -> Option<Vec<Vertex>> {
        let mut intersections = vec![];

        for self_ls in self.line_segments() {
            for other_ls in other.line_segments() {
                if let Some(intersection) = self_ls.intersect(&other_ls) {
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

struct Beacon {
    pos: Vertex,
}

struct Sensor {
    pos: Vertex,
    beacon: Beacon,
}

impl Sensor {
    fn from(input: &str) -> Self {
        let mut input = input.split(['=', ',', ':']);

        let pos = Vertex {
            x: input.nth(1).unwrap().parse::<f64>().unwrap(),
            y: input.nth(1).unwrap().parse::<f64>().unwrap(),
        };

        let beacon = Beacon { 
            pos: Vertex {
                x: input.nth(1).unwrap().parse::<f64>().unwrap(),
                y: input.nth(1).unwrap().parse::<f64>().unwrap(),
            }
        };

        Self { pos, beacon }

    }

    fn beacon_distance(&self) -> u64 {
        self.pos.manhattan_distance(self.beacon.pos)
    }

    fn can_see(&self, coord: Vertex) -> bool {
        self.pos.manhattan_distance(coord) <= self.beacon_distance()
    }

}

fn day15_part1(input: &str, y: i64) -> usize {
    let sensors: Vec<Sensor> = input
        .trim()
        .lines()
        .map(Sensor::from)
        .collect();

    let squares: Vec<Square> = sensors
        .iter()
        .map(Square::from)
        .collect();

    // represent the given y as an infinite line segment
    let row = LineSegment { 
        slope: 0.0,
        constant: y as f64,
        x_min: -f64::INFINITY,
        x_max: f64::INFINITY
    };


    // find ranges of occupied y by determining intersections of y with each square
    let mut ranges: Vec<(i64, i64)> = vec![];

    for square in &squares {
        if let Some(intersections) = square.intersect_line_segment(&row) {
            let mut range = (
                intersections
                    .iter()
                    .map(|intersection| intersection.x as i64).min().unwrap(),
                intersections
                    .iter()
                    .map(|intersection| intersection.x as i64).max().unwrap(),
            );

            if range.0.abs_diff(range.1) != 0 {
                range.0 += 1;
                range.1 -= 1;
            }

            ranges.push(range);
        }
    }

    ranges.sort_by(|a, b| b.cmp(a));

    // join ranges
    let mut joined_ranges: Vec<(i64, i64)> = vec![];

    while let Some(mut range) = ranges.pop() {
        while let Some(inner_range) = ranges.last() {
            if range.1 >= inner_range.0 {
                range.1 = range.1.max(inner_range.1);
                ranges.pop();
            } else {
                break;
            }
        }
        
        joined_ranges.push(range)
    }

    // sum joined ranges
    joined_ranges
        .iter()
        .map(|joined_range| joined_range.1 - joined_range.0)
        .sum::<i64>() as usize
}

pub fn day15_part2(input: &str, range: (f64, f64)) -> i64 {
    let sensors: Vec<Sensor> = input
        .trim()
        .lines()
        .map(Sensor::from)
        .collect();

    let squares: Vec<Square> = sensors
        .iter()
        .map(Square::from)
        .collect();

    let range = Square {
        vertices: [
            Vertex { x: range.1,  y: range.0 },
            Vertex { x: range.1,  y: range.1 },
            Vertex { x: range.0,  y: range.1 },
            Vertex { x: range.0,  y: range.0 },
        ]
    };

    // for each square:
    // 1. find all intersections with other squares
    //
    // for all intersection:
    // 1. return the first intersection that does not lie inside any square and
    //    which lies within the requested range
    for square in &squares {
        for other_square in &squares {
            if let Some(intersections) = square.intersect_square(other_square) {
                for intersection in intersections {
                    if range.contains(intersection)
                        && squares
                            .iter()
                            .all(|square| !square.contains(intersection)
                    ) {
                        let freq = intersection.x * 4000000.0 + intersection.y;
                        return freq as i64;
                    }
                }
            }
        }
    }

    0
}

pub fn part1(input: &str) -> usize {
    day15_part1(input, 2000000)
}

pub fn part2(input: &str) -> i64 {
    day15_part2(input, (0.0, 4000000.0))
}

#[cfg(test)]
mod tests {
    use super::{day15_part1, day15_part2};

    const TEST_INPUT: &str = 
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
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
        assert_eq!(day15_part1(TEST_INPUT, 10), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(day15_part2(TEST_INPUT, (0.0, 20.0)), 56000011);
    }
}

