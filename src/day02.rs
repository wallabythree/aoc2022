pub fn part1(input: &str) -> i64 {
    let bytes = input.as_bytes();
    let rows: usize = bytes.len() / 4;

    let mut score: i64 = 0;

    for row in 0..rows {
        let i = row * 4;
        let player = (bytes[i + 2] - 87) as i64;
        let opponent = (bytes[i] - 64) as i64;

        score += player;

        let mut round = 0;

        match opponent {
            1 => match player {
                2 => round = 6,
                1 => round = 3,
                _ => ()
            },
            2 => match player {
                3 => round = 6,
                2 => round = 3,
                _ => ()
            }
            3 => match player {
                1 => round = 6,
                3 => round = 3,
                _ => ()
            }
            _ => ()
        }

        score += round;
    }
    score
}

pub fn part2(input: &str) -> i64 {
    let bytes = input.as_bytes();
    let rows: usize = bytes.len() / 4;

    let mut score: i64 = 0;

    for row in 0..rows {
        let i = row * 4;
        let opponent = (bytes[i] - 64) as i64;
        let round = ((bytes[i + 2] - 87) as i64 - 1) * 3;

        score += round;

        let mut player = 0;

        match opponent {
            1 => match round {
                6 => player = 2,
                3 => player = 1,
                _ => player = 3
            },
            2 => match round {
                6 => player = 3,
                3 => player = 2,
                _ => player = 1
            },
            3 => match round {
                6 => player = 1,
                3 => player = 3,
                _ => player = 2
            },
            _ => ()
        }

        score += player;
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

