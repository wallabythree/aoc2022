pub fn part1(input: &str) -> i64 {
    let bytes = input.as_bytes();
    let rows: usize = bytes.len() / 4;

    let mut score: i64 = 0;

    for row in 0..rows {
        let i = row * 4;

        let opponent = bytes[i] as i64 - 64;
        let player = bytes[i + 2] as i64 - 87;
        let round = (player - opponent + 1).rem_euclid(3) * 3; 

        score += player + round;
    }
    score
}

pub fn part2(input: &str) -> i64 {
    let bytes = input.as_bytes();
    let rows: usize = bytes.len() / 4;

    let mut score: i64 = 0;

    for row in 0..rows {
        let i = row * 4;
        let opponent = bytes[i] as i64 - 64;
        let round = (bytes[i + 2] as i64 - 87 - 1) * 3;
        let player = (opponent + bytes[i + 2] as i64 - 87 - 2 - 1)
            .rem_euclid(3) + 1; 

        score += round + player;
    }
    score
}

#[cfg(test)]
mod tests {
    use crate::day02::{part1, part2};

    const TEST_INPUT: &str = "A Y\nB X\nC Z\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 15);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 12);
    }
}

