trait Priority {
    fn priority(&self) -> u8;
}

impl Priority for u8 {
    fn priority(&self) -> u8 {
        if self.is_ascii_lowercase() {
            *self as u8 - b'a' + 1
        } else {
            *self as u8 - b'A' + 27
        }
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .split('\n')
        .map(|sack| {
            let front = sack[..sack.len() / 2].as_bytes();
            let rear = sack[sack.len() / 2..].as_bytes();

            // hash table
            let mut table = [0u8; 256];

            for c in front {
                table[*c as usize] += 1;
            }

            for c in rear {
                if table[*c as usize] != 0 {
                    return c.priority() as u64;
                }
            }

            0
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .trim()
        .split('\n')
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {

            // hash tables
            let mut tables = [[0u8; 256]; 2];

            for (i, sack) in group.iter().take(2).enumerate() {
                for c in sack.as_bytes() {
                    tables[i][*c as usize] = 1;
                }
            }

            for c in group.get(2).unwrap().as_bytes() {
                if tables[0][*c as usize] != 0 && tables[1][*c as usize] != 0 {
                    return c.priority() as u64;
                }
            }

            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day03::{part1, part2};

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                              jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                              PmmdzqPrVvPwwTWBwg\n\
                              wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                              ttgJtRGJQctTZtZT\n\
                              CrZsJsPPZsGzwwsLwLmpwMDw\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 157);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 70);
    }
}

