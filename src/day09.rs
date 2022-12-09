#[derive(Clone)]
#[derive(Copy)]
struct Knot {
    x: i64,
    y: i64 
}

struct Rope {
    knots: Vec<Knot>
}

impl Rope {
    fn step(&mut self, dir: u8) {
        let head = &mut self.knots[0];

        match dir {
            b'U' => head.y -= 1,
            b'D' => head.y += 1,
            b'L' => head.x -= 1,
            b'R' => head.x += 1,
            _ => ()
        }

        for i in 1..self.knots.len() {
            let head = self.knots[i - 1];
            let tail = &mut self.knots[i];

            match (head.x - tail.x, head.y - tail.y) {
                (-2, 0) => tail.x -= 1,
                (2, 0) => tail.x += 1,
                (0, -2) => tail.y -= 1,
                (0, 2) => tail.y += 1,

                (1, -2) | (2, -1) | (2, -2) => {
                    tail.x += 1;
                    tail.y -= 1;
                },
                (-1, -2) | (-2, -1) | (-2, -2) => {
                    tail.x -= 1;
                    tail.y -= 1;
                },
                (-1, 2) | (-2, 1) | (-2, 2) => {
                    tail.x -= 1;
                    tail.y += 1;
                },
                (1, 2) | (2, 1) | (2,2) => {
                    tail.x += 1;
                    tail.y += 1;
                },
                _ => ()
            }
        }
    }
}

fn visited(input: &str, rope_len: usize) -> usize {
    let mut visited = [[false; 1000]; 1000];
    
    let mut rope = Rope {
        knots: vec![Knot { x: 0, y: 0 }; rope_len]
    };

    input
        .trim()
        .split([' ', '\n'])
        .collect::<Vec<_>>()
        .chunks(2)
        .for_each(|instr| {
            let (dir, dist) = (
                instr[0].as_bytes()[0],
                instr[1].parse::<usize>().unwrap()
            );

            for _ in 0..dist {
                rope.step(dir);

                let (x, y) = (
                    (rope.knots[rope_len - 1].y + 500) as usize,
                    (rope.knots[rope_len - 1].x + 500) as usize
                );

                visited[y][x] = true;
            }

        });

    visited
        .iter()
        .map(|row| row
             .iter()
             .filter(|&pos| *pos)
             .count())
        .sum()
}

pub fn part1(input: &str) -> usize {
    visited(input, 2)
}

pub fn part2(input: &str) -> usize {
    visited(input, 10)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT_1: &str = "R 4\n\
                                U 4\n\
                                L 3\n\
                                D 1\n\
                                R 4\n\
                                D 1\n\
                                L 5\n\
                                R 2\n";

    const TEST_INPUT_2: &str = "R 5\n\
                                U 8\n\
                                L 8\n\
                                D 3\n\
                                R 17\n\
                                D 10\n\
                                L 25\n\
                                U 20\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 13);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), 36);
    }
}

