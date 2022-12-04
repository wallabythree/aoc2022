// solution template

pub fn part1(input: &str) -> u64 {
    input
        .trim()
        .split(['\n', '-', ','])
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(4)
        .filter(|row| {
            let (l, r) = ((row[0], row[1]), (row[2], row[3]));

            (l.0 >= r.0 && l.1 <= r.1) || (r.0 >= l.0 && r.1 <= l.1)
        })
        .count() as u64
}

pub fn part2(input: &str) -> u64 {
    input
        .trim()
        .split(['\n', '-', ','])
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(4)
        .filter(|row| {
            let (l, r) = ((row[0], row[1]), (row[2], row[3]));

            (l.0 >= r.0 && l.0 <= r.1)
            || (r.0 >= l.0 && r.0 <= l.1)
            || (l.1 <= r.1 && l.1 >= r.0)
            || (r.1 <= l.1 && r.1 >= l.0)
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use crate::day04::{part1, part2};

    const TEST_INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n\
                              2-8,3-7\n6-6,4-6\n2-6,4-8\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 2);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4);
    }
}

