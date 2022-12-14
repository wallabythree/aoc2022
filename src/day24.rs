use std::collections::HashMap;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashSet;
use std::collections::VecDeque;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Blizzard {
    start: (i64, i64),
    direction: Direction,
}

impl Blizzard {
    fn pos(&self, minute: usize) -> (i64, i64) {
        match self.direction {
            Direction::Up => (self.start.0, self.start.1 - minute as i64),
            Direction::Down => (self.start.0, self.start.1 + minute as i64),
            Direction::Left => (self.start.0 - minute as i64, self.start.1),
            Direction::Right => (self.start.0 + minute as i64, self.start.1),
        }
    }
}

struct Grid {
    start: (i64, i64),
    end: (i64, i64),
    width: usize,
    height: usize,
    blizzards: Vec<Blizzard>,
    blizzard_cache: HashMap<usize, HashSet<(i64, i64)>>
}

impl Grid {
    fn from(input: &str) -> Self {
        let rows: Vec<&str> = input
            .lines()
            .collect();

        let start = (input.find('.').unwrap() as i64, 0);
        let end = (
            rows.last().unwrap().rfind('.').unwrap() as i64,
            rows.len() as i64 - 1
        );

        let width = rows.first().unwrap().len();
        let height = (end.1 - start.1) as usize + 1;

        let mut blizzards = vec![];

        for (y, row) in rows.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '#' || c == '.' {
                    continue;
                }

                let direction = match c {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => panic!(),
                };

                let blizzard = Blizzard {
                    start: (x as i64, y as i64),
                    direction 
                };

                blizzards.push(blizzard);
            }
        }

        let blizzard_cache = HashMap::new();

        Self { start, end, width, height, blizzards, blizzard_cache }
    }

    // are we still on the map or have we hit a wall?
    fn within_bounds(&self, square: (i64, i64)) -> bool {
        square.0 > 0
            && square.0 < self.width as i64 - 1
            && (square.1 > 0 || square == self.start)
            && (square.1 < self.height as i64 - 1 || square == self.end)
    }

    fn traverse(
        &mut self,
        start: (i64, i64),
        end: (i64, i64),
        start_minute: usize
    ) -> Result<usize, ()> {

        let start_node = (start, start_minute);

        let mut queue: VecDeque<((i64, i64), usize)> = VecDeque::new();
        queue.push_back(start_node);

        let mut visited = HashSet::new();
        visited.insert(start_node);

        while let Some(node) = queue.pop_front() {
            if node.0 == end {
                return Ok(node.1);
            }

            // determine next minute's blizzards, if we haven't already done so
            if let Vacant(e) = self.blizzard_cache.entry(node.1 + 1) {
                let mut next_blizzards = HashSet::new();

                for blizzard in &self.blizzards {
                    let mut blizzard_pos = blizzard.pos(node.1 + 1);

                    blizzard_pos.0 = (blizzard_pos.0 - 1)
                        .rem_euclid(self.width as i64 - 2) + 1;
                    blizzard_pos.1 = (blizzard_pos.1 - 1)
                        .rem_euclid(self.height as i64 - 2) + 1;
                    next_blizzards.insert(blizzard_pos);
                }

                e.insert(next_blizzards);
            }

            let wait = (node.0, node.1 + 1);
            let up = ((node.0.0, node.0.1 - 1), node.1 + 1);
            let down = ((node.0.0, node.0.1 + 1), node.1 + 1);
            let left = ((node.0.0 - 1, node.0.1), node.1 + 1);
            let right = ((node.0.0 + 1, node.0.1), node.1 + 1);

            let next_nodes = [wait, up, down, left, right];

            for next in next_nodes.iter() {
                if self.within_bounds(next.0)
                   && !visited.contains(next)
                   && !self
                       .blizzard_cache
                       .get(&next.1)
                       .unwrap()
                       .contains(&next.0) {
                    queue.push_back(*next);
                    visited.insert(*next);
                }
            }
        }

        Err(())
    }
}

pub fn part1(input: &str) -> usize {
    let mut mountain = Grid::from(input);

    mountain.traverse(mountain.start, mountain.end, 0).unwrap()
}

#[allow(clippy::let_and_return)]
pub fn part2(input: &str) -> usize {
    let mut mountain = Grid::from(input);

    let there = mountain.traverse(mountain.start, mountain.end, 0).unwrap();
    let back = mountain.traverse(mountain.end, mountain.start, there).unwrap();
    let again = mountain.traverse(mountain.start, mountain.end, back).unwrap();

    again
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "#.######\n\
                              #>>.<^<#\n\
                              #.<..<<#\n\
                              #>v.><>#\n\
                              #<^v^^>#\n\
                              ######.#\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 18);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 54);
    }
}

