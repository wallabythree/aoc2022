trait Priority {
    fn priority(&self) -> u64;
}

impl Priority for char {
    fn priority(&self) -> u64 {
        if self.is_lowercase() {
            *self as u64 - 'a' as u64 + 1
        } else {
            *self as u64 - 'A' as u64 + 27
        }
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .split('\n')
        .map(|sack| {
            let front = &sack[..sack.len() / 2];
            let rear = &sack[sack.len() / 2..];

            for c in front.chars() {
                if rear.contains(c) {
                    return c.priority();
                }
            }

            0
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let elves: Vec<&str> = input.split('\n').collect();
    let groups = elves.chunks(3);

    let mut sum: u64 = 0;

    for group in groups {
        for c in group[0].chars() {
            if group[1].contains(c) && group[2].contains(c) {
                sum += c.priority();
                break;
            }
        }
    }

    sum
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

