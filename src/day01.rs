pub fn part1(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|elf| {
            elf
                .split('\n')
                .map(|fruit| fruit.parse::<u64>().unwrap_or(0))
                .sum::<u64>()
        })
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> u64 {
    let mut ranked = input
        .split("\n\n")
        .map(|elf| {
            elf
                .split('\n')
                .map(|fruit| fruit.parse::<u64>().unwrap_or(0))
                .sum::<u64>()
        })
        .collect::<Vec<u64>>();

    ranked.sort_by(|a, b| b.cmp(a));

    ranked[..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::day01;

    const TEST_INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\
                              \n\n7000\n8000\n9000\n\n10000";

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

