use std::collections::HashMap;
use std::collections::HashSet;

struct Board {
    elves: HashSet<(i64, i64)>,
    rules: Vec<Rule>,
    rounds: usize,
}

type Rule = fn(&Board, (i64, i64)) -> Option<(i64, i64)>;

impl Board {
    fn from(input: &str) -> Self {
        let elves: HashSet<(i64, i64)> = input
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row
                    .chars()
                    .enumerate()
                    .filter_map(move |(x, c)| {
                        if c == '#' {
                            Some((x as i64, y as i64))
                        } else {
                            None
                        }
                    })
            })
            .collect();

        let mut rules: Vec<Rule> = vec![];

        // North rule
        rules.push(|board: &Board, elf: (i64, i64)| {
            if board.elves.get(&(elf.0 - 1, elf.1 - 1)).is_none()
               && board.elves.get(&(elf.0, elf.1 - 1)).is_none()
               && board.elves.get(&(elf.0 + 1, elf.1 - 1)).is_none() {
                   Some((elf.0, elf.1 - 1))
               } else {
                   None
               }
        });

        // South rule
        rules.push(|board: &Board, elf: (i64, i64)| {
            if board.elves.get(&(elf.0 - 1, elf.1 + 1)).is_none()
               && board.elves.get(&(elf.0, elf.1 + 1)).is_none()
               && board.elves.get(&(elf.0 + 1, elf.1 + 1)).is_none() {
                   Some((elf.0, elf.1 + 1))
               } else {
                   None
               }
        });

        // West rule
        rules.push(|board: &Board, elf: (i64, i64)| {
            if board.elves.get(&(elf.0 - 1, elf.1 - 1)).is_none()
               && board.elves.get(&(elf.0 - 1, elf.1)).is_none()
               && board.elves.get(&(elf.0 - 1, elf.1 + 1)).is_none() {
                   Some((elf.0 - 1, elf.1))
               } else {
                   None
               }
        });

        // East rule
        rules.push(|board: &Board, elf: (i64, i64)| {
            if board.elves.get(&(elf.0 + 1, elf.1 - 1)).is_none()
               && board.elves.get(&(elf.0 + 1, elf.1)).is_none()
               && board.elves.get(&(elf.0 + 1, elf.1 + 1)).is_none() {
                   Some((elf.0 + 1, elf.1))
               } else {
                   None
               }
        });

        Self { elves, rules, rounds: 0 }
    }

    fn proposed_move(&self, elf: (i64, i64)) -> Option<(i64, i64)> {
       // don't propose move if elf has no neighbours
        if self.elves.get(&(elf.0 - 1, elf.1 - 1)).is_none()
           && self.elves.get(&(elf.0, elf.1 - 1)).is_none()
           && self.elves.get(&(elf.0 + 1, elf.1 - 1)).is_none()
           && self.elves.get(&(elf.0 - 1, elf.1)).is_none()
           && self.elves.get(&(elf.0 + 1, elf.1)).is_none()
           && self.elves.get(&(elf.0 - 1, elf.1 + 1)).is_none()
           && self.elves.get(&(elf.0, elf.1 + 1)).is_none()
           && self.elves.get(&(elf.0 + 1, elf.1 + 1)).is_none() {
            return None;
        }

        // iterate over rules until a valid move is found
        for i in 0..self.rules.len() {
            let j = (i + self.rounds).rem_euclid(self.rules.len());
            let rule = self.rules[j];
            
            if let Some(dest) = rule(self, elf) {
                return Some(dest);
            }
        }

        None
    }

    fn play_round(&mut self) {
        let mut moves: HashMap<(i64, i64), (i64, i64)> = HashMap::new();
        let mut dest_freq: HashMap<(i64, i64), usize> = HashMap::new();

        for elf in self.elves.iter() {
            if let Some(dest) = self.proposed_move(*elf) {
                moves.insert(*elf, dest);

                if let Some(freq) = dest_freq.get(&dest) {
                    dest_freq.insert(dest, freq + 1);
                } else {
                    dest_freq.insert(dest, 1);
                }
            }
        }

        let mut new_positions: HashSet<(i64, i64)> = HashSet::new();

        for elf in self.elves.iter() {
            if let Some(dest) = moves.get(elf) {
                if *dest_freq.get(dest).unwrap() == 1 {
                    new_positions.insert(*dest);
                } else {
                    new_positions.insert(*elf);
                }
            } else {
                new_positions.insert(*elf);
            }
        }

        self.elves = new_positions;
        self.rounds += 1;
    }

    fn ground_tiles(&self) -> usize {
        let min_x = self.elves.iter().map(|elf| elf.0).min().unwrap();
        let max_x = self.elves.iter().map(|elf| elf.0).max().unwrap() + 1;
        let min_y = self.elves.iter().map(|elf| elf.1).min().unwrap();
        let max_y = self.elves.iter().map(|elf| elf.1).max().unwrap() + 1;

        (max_x - min_x) as usize * (max_y - min_y) as usize - self.elves.len()
    }
}

pub fn part1(input: &str) -> usize {
    let mut board = Board::from(input);

    for _ in 0..10 {
        board.play_round();
    }

    board.ground_tiles()
}

pub fn part2(input: &str) -> usize {
    let mut board = Board::from(input);

    let mut prev_round = board.elves.clone();
    board.play_round();

    while board.elves != prev_round {
        prev_round = board.elves.clone();
        board.play_round();
    }

    board.rounds
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "....#..\n\
                              ..###.#\n\
                              #...#.#\n\
                              .#...##\n\
                              #.###..\n\
                              ##.#.##\n\
                              .#..#..\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 110);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 20);
    }
}

