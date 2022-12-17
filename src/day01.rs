pub fn part1(input: &str) -> u64 {
    let mut max: u64 = 0;
    let mut cur: u64 = 0;

    input
        .split('\n')
        .for_each(|fruit| {
            if fruit.is_empty() {
                if cur > max {
                    max = cur;
                }

                cur = 0;

            } else {
                cur += fruit.parse::<u64>().unwrap_or(0);
            }
        });

    max
}

pub fn part2(input: &str) -> u64 {
    let mut top = [0u64; 3];
    let mut cur: u64 = 0;

    input
        .split('\n')
        .for_each(|fruit| {
            if fruit.is_empty() {
                for i in 0..top.len() {
                    if cur > top[i] {
                        for j in ((i + 1)..top.len()).rev() {
                            top[j] = top[j - 1];
                        }

                        top[i] = cur;
                        break;
                    }
                }

                cur = 0;
            } else {
                cur += fruit.parse::<u64>().unwrap_or(0);
            }
        });

    top.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::day01;

    const TEST_INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\
                              \n\n7000\n8000\n9000\n\n10000\n";

    const BYE: [u8; 10] = [0x01; 10];

    #[test]
    fn test_part1() {
        assert_eq!(day01::part1(TEST_INPUT), 24000);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(day01::part2(TEST_INPUT), 45000);
    }

}

