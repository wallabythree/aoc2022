//use std::collections::VecDeque;

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
struct Coordinate {
    val: i64,
    pos: usize,
}

pub fn part1(input: &str) -> i64 {
    let coordinates: Vec<Coordinate> = input
        .lines()
        .enumerate()
        .map(|(pos, coordinate)| Coordinate { pos, val: coordinate.parse().unwrap() } )
        .collect();

    let mut new_coordinates = coordinates.clone();

    for coordinate in coordinates.iter() {
        let pos = new_coordinates
            .iter()
            .position(|new_coord| new_coord == coordinate)
            .unwrap();

        new_coordinates.remove(pos);

        let mut new_pos = (pos as i64 + coordinate.val)
            .rem_euclid(new_coordinates.len() as i64) as usize;

        if new_pos == 0 {
            new_pos = new_coordinates.len();
        }

        new_coordinates.insert(new_pos, *coordinate);
    }

    let zero_pos = new_coordinates.iter().position(|coord| coord.val == 0).unwrap();

    new_coordinates[(zero_pos + 1000).rem_euclid(new_coordinates.len())].val
    + new_coordinates[(zero_pos + 2000).rem_euclid(new_coordinates.len())].val
    + new_coordinates[(zero_pos + 3000).rem_euclid(new_coordinates.len())].val
}

pub fn part2(input: &str) -> i64 {
    let coordinates: Vec<Coordinate> = input
        .lines()
        .enumerate()
        .map(|(pos, coordinate)| Coordinate { 
            pos,
            val: coordinate.parse::<i64>().unwrap() * 811589153
        })
        .collect();

    let mut new_coordinates = coordinates.clone();

    for _ in 0..10 {
        for coordinate in coordinates.iter() {
            let pos = new_coordinates
                .iter()
                .position(|new_coord| new_coord == coordinate)
                .unwrap();

            new_coordinates.remove(pos);

            let mut new_pos = (pos as i64 + coordinate.val)
                .rem_euclid(new_coordinates.len() as i64) as usize;

            if new_pos == 0 {
                new_pos = new_coordinates.len();
            }

            new_coordinates.insert(new_pos, *coordinate);
        }
    }

    let zero_pos = new_coordinates
        .iter()
        .position(|coord| coord.val == 0)
        .unwrap();

    new_coordinates[(zero_pos + 1000).rem_euclid(new_coordinates.len())].val
    + new_coordinates[(zero_pos + 2000).rem_euclid(new_coordinates.len())].val
    + new_coordinates[(zero_pos + 3000).rem_euclid(new_coordinates.len())].val
}


#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "1\n\
                              2\n\
                              -3\n\
                              3\n\
                              -2\n\
                              0\n\
                              4\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1623178306);
    }
}

