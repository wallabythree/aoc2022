pub fn part1(input: &str) -> String {
    let mut sum = input
        .lines()
        .map(|snafu| {
            snafu
                .chars()
                .rev()
                .enumerate()
                .map(|(i, c)| {
                    (match c {
                        '=' => -2,
                        '-' => -1,
                        '0' => 0,
                        '1' => 1,
                        '2' => 2,
                        _ => panic!(),
                    }) * 5i64.pow(i as u32) as i64
                })
                .sum::<i64>()
        })
        .sum::<i64>();

        let mut snafu = vec![];
        let mut i = 1;

        while sum != 0 {
            let num = sum % 5i64.pow(i);
            sum -= num;
            snafu.push(num / 5i64.pow(i - 1));
            i += 1;
        }

        for i in 0..snafu.len() {
            let carry = (snafu[i] + 2) / 5;
            let remainder = snafu[i] - 5 * carry;

            snafu[i] = remainder;
            
            if carry != 0 {
                if i + 1 < snafu.len() {
                    snafu[i + 1] += carry;
                } else {
                    snafu.push(carry);
                }
            }
        }

        snafu
            .iter()
            .rev()
            .map(|num| {
                match num {
                    -2 => '=',
                    -1 => '-',
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    _ => panic!(),
                }
            })
            .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::part1;

    const TEST_INPUT: &str = "1=-0-2\n\
                              12111\n\
                              2=0=\n\
                              21\n\
                              2=01\n\
                              111\n\
                              20012\n\
                              112\n\
                              1=-1=\n\
                              1-12\n\
                              12\n\
                              1=\n\
                              122\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "2=-1=0");
    }
}

