fn find_marker(input: &str, packet_len: usize) -> Result<usize, ()> {
    for i in packet_len..(input.len()) {
        let chunk = &input.as_bytes()[(i - packet_len)..i];
        let mut packet_start = true;

        // create hash table
        let mut table = [0u8; u8::MAX as usize];

        for b in chunk.iter() {
            if table[*b as usize] != 0 {
                packet_start = false;
                break;
            }
        
            // populate hash table
            table[*b as usize] += 1;
        }

        if packet_start {
            return Ok(i);
        }
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

