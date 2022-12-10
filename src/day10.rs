pub fn part1(input: &str) -> i64 {
    let mut cycle = 0;
    let mut acc = 1;

    let mut signal_strength = 0;

    input
        .trim()
        .split([' ', '\n'])
        .for_each(|arg| {
            cycle += 1;

            if cycle == 20 || (cycle - 20) % 40 == 0 {
                signal_strength += cycle * acc;
            }

            let op = arg.chars().next().unwrap();

            if op != 'a' && op != 'n' {
                let addend = arg.parse::<i64>().unwrap();
                acc += addend;
            }
        });

    signal_strength
}

pub fn part2(input: &str) -> String {
    let mut cycle = 0;
    let mut acc = 1;

    let mut pixbuf = String::new();

    input
        .trim()
        .split([' ', '\n'])
        .for_each(|arg| {
            cycle += 1;

            let column: i64 = (cycle - 1) % 40;

            if column == 0 && cycle > 1 {
                pixbuf.push('\n')
            }

            if (column - acc).abs() < 2 {
                pixbuf.push('#');
            } else {
                pixbuf.push('.');
            }

            let op = arg.chars().next().unwrap();

            if op != 'a' && op != 'n' {
                let addend = arg.parse::<i64>().unwrap();
                acc += addend;
            }
        });

    pixbuf
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "addx 15\n\
                              addx -11\n\
                              addx 6\n\
                              addx -3\n\
                              addx 5\n\
                              addx -1\n\
                              addx -8\n\
                              addx 13\n\
                              addx 4\n\
                              noop\n\
                              addx -1\n\
                              addx 5\n\
                              addx -1\n\
                              addx 5\n\
                              addx -1\n\
                              addx 5\n\
                              addx -1\n\
                              addx 5\n\
                              addx -1\n\
                              addx -35\n\
                              addx 1\n\
                              addx 24\n\
                              addx -19\n\
                              addx 1\n\
                              addx 16\n\
                              addx -11\n\
                              noop\n\
                              noop\n\
                              addx 21\n\
                              addx -15\n\
                              noop\n\
                              noop\n\
                              addx -3\n\
                              addx 9\n\
                              addx 1\n\
                              addx -3\n\
                              addx 8\n\
                              addx 1\n\
                              addx 5\n\
                              noop\n\
                              noop\n\
                              noop\n\
                              noop\n\
                              noop\n\
                              addx -36\n\
                              noop\n\
                              addx 1\n\
                              addx 7\n\
                              noop\n\
                              noop\n\
                              noop\n\
                              addx 2\n\
                              addx 6\n\
                              noop\n\
                              noop\n\
                              noop\n\
                              noop\n\
                              noop\n\
                              addx 1\n\
                              noop\n\
                              noop\n\
                              addx 7\n\
                              addx 1\n\
                              noop\n\
                              addx -13\n\
                              addx 13\n\
                              addx 7\n\
                              noop\n\
                              addx 1\n\
                              addx -33\n\
                              noop\n\
                              noop\n\
                              noop\n\
                              addx 2\n\
                              noop\n\
                              noop\n\
                              noop\n\
                              addx 8\n\
                              noop\n\
                              addx -1\n\
                              addx 2\n\
                              addx 1\n\
                              noop\n\
                              addx 17\n\
                              addx -9\n\
                              addx 1\n\
                              addx 1\n\
                              addx -3\n\
                              addx 11\n\
                              noop\n\
                              noop\n\
                              addx 1\n\
                              noop\n\
                              addx 1\n\
                              noop\n\
                              noop\n\
                              addx -13\n\
                              addx -19\n\
                              addx 1\n\
                              addx 3\n\
                              addx 26\n\
                              addx -30\n\
                              addx 12\n\
                              addx -1\n\
                              addx 3\n\
                              addx 1\n\
                              noop\n\
                              noop\n\
                              noop\n\
                              addx -9\n\
                              addx 18\n\
                              addx 1\n\
                              addx 2\n\
                              noop\n\
                              noop\n\
                              addx 9\n\
                              noop\n\
                              noop\n\
                              noop\n\
                              addx -1\n\
                              addx 2\n\
                              addx -37\n\
                              addx 1\n\
                              addx 3\n\
                              noop\n\
                              addx 15\n\
                              addx -21\n\
                              addx 22\n\
                              addx -6\n\
                              addx 1\n\
                              noop\n\
                              addx 2\n\
                              addx 1\n\
                              noop\n\
                              addx -10\n\
                              noop\n\
                              noop\n\
                              addx 20\n\
                              addx 1\n\
                              addx 2\n\
                              addx 2\n\
                              addx -6\n\
                              addx -11\n\
                              noop\n\
                              noop\n\
                              noop\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13140);
    }

    const PART2_TEST_OUTPUT: &str = "##..##..##..##..##..##..##..##..##..##..\n\
                                     ###...###...###...###...###...###...###.\n\
                                     ####....####....####....####....####....\n\
                                     #####.....#####.....#####.....#####.....\n\
                                     ######......######......######......####\n\
                                     #######.......#######.......#######.....";

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), PART2_TEST_OUTPUT);
    }
}

