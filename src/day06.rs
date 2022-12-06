fn find_marker(input: &str, packet_len: usize) -> Result<usize, ()> {
    let bytes = input.as_bytes();

    // create hash table
    let mut table = [0u8; u8::MAX as usize];
    // count hash collisions
    let mut collisions = 0;

    for (i, b) in bytes.iter().enumerate() {
        table[*b as usize] += 1;

        if table[*b as usize] > 1 {
            collisions += 1;
        }

        if i < packet_len {
            continue;
        }

        let p = bytes[i - packet_len]; // byte leaving packet scope

        if table[p as usize] > 1 {
            collisions -= 1;

            if collisions == 0 {
                return Ok(i + 1);
            }
        }

        table[p as usize] -= 1;
    }

    Err(())
}

pub fn part1(input: &str) -> usize {
    find_marker(input, 4).unwrap()
}

pub fn part2(input: &str) -> usize {
    find_marker(input, 14).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day06::{part1, part2};

    const TEST_INPUTS: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
    ];

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUTS[0]), 7);
        assert_eq!(part1(TEST_INPUTS[1]), 5);
        assert_eq!(part1(TEST_INPUTS[2]), 6);
        assert_eq!(part1(TEST_INPUTS[3]), 10);
        assert_eq!(part1(TEST_INPUTS[4]), 11);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUTS[0]), 19);
        assert_eq!(part2(TEST_INPUTS[1]), 23);
        assert_eq!(part2(TEST_INPUTS[2]), 23);
        assert_eq!(part2(TEST_INPUTS[3]), 29);
        assert_eq!(part2(TEST_INPUTS[4]), 26);
    }
}

