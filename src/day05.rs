pub fn part1(input: &str) -> u64 {
    input.len() as u64
}

pub fn part2(input: &str) -> u64 {
    input.len() as u64
}

#[cfg(test)]
mod tests {
    use crate::day05::{part1, part2};

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 0);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}

