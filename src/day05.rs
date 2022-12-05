pub fn part1(input: &str) -> String {
    let (stacks_s, movs_s) = input
        .split_once("\n\n")
        .unwrap();

    // create stacks
    let stack_count = stacks_s.find('\n').unwrap() / 4 + 1;
    let mut stacks: Vec<Vec<u8>> = vec![Vec::new(); stack_count];

    // populate stacks
    stacks_s
        .split('\n')
        .rev()
        .skip(1)
        .for_each(|row_s| {
            let row = row_s.as_bytes();

            for i in 0..stack_count {
                if row[i * 4 + 1] != b' ' {
                    stacks[i].push(row[i * 4 + 1]);
                }
            }
        });

    // execute moves
    movs_s
        .trim()
        .split([' ', '\n'])
        .collect::<Vec<_>>()
        .chunks(6)
        .for_each(|instr| {
            let (count, src, dst) = (
                instr.get(1).unwrap().parse::<usize>().unwrap(),
                instr.get(3).unwrap().parse::<usize>().unwrap() - 1,
                instr.get(5).unwrap().parse::<usize>().unwrap() - 1,
            );

            for _ in 0..count {
                let container = stacks[src].pop().unwrap();
                stacks[dst].push(container);
            }

        });

    let mut top = String::new(); 
    for stack in stacks {
        top.push(*stack.last().unwrap() as char);
    }

    top
}

pub fn part2(input: &str) -> String {
    let (stacks_s, movs_s) = input
        .split_once("\n\n")
        .unwrap();

    // create stacks
    let stack_count = stacks_s.find('\n').unwrap() / 4 + 1;
    let mut stacks: Vec<Vec<u8>> = vec![Vec::new(); stack_count];

    // populate stacks
    stacks_s
        .split('\n')
        .rev()
        .skip(1)
        .for_each(|row_s| {
            let row = row_s.as_bytes();

            for i in 0..stack_count {
                if row[i * 4 + 2] != b' ' {
                    stacks[i].push(row[i * 4 + 1]);
                }
            }
        });

    // execute moves
    movs_s
        .trim()
        .split([' ', '\n'])
        .collect::<Vec<_>>()
        .chunks(6)
        .for_each(|instr| {
            let (count, src, dst) = (
                instr.get(1).unwrap().parse::<usize>().unwrap(),
                instr.get(3).unwrap().parse::<usize>().unwrap() - 1,
                instr.get(5).unwrap().parse::<usize>().unwrap() - 1,
            );


            let leave = stacks[src].len() - count;
            let mut containers = stacks[src].split_off(leave);
            stacks[dst].append(&mut containers);

        });

    let mut top = String::new(); 
    for stack in stacks {
        top.push(*stack.last().unwrap() as char);
    }

    top
}

#[cfg(test)]
mod tests {
    use crate::day05::{part1, part2};

    const TEST_INPUT: &str = "    [D]    \n\
                              [N] [C]    \n\
                              [Z] [M] [P]\n 1   2   3 \n\n\
                              move 1 from 2 to 1\n\
                              move 3 from 1 to 3\n\
                              move 2 from 2 to 1\n\
                              move 1 from 1 to 2\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "CMZ");
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "MCD");
    }
}

