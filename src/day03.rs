pub fn part1(input: &str) -> i64 {
    input.len() as i64
}

pub fn part2(input: &str) -> i64 {
    input.len() as i64
}

#[cfg(test)]
mod tests {
    use crate::day03::{part1, part2};

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

