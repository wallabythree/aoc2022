use core::cmp::Ordering;

// recursive list
#[derive(Clone,Eq)]
enum ListItem {
    Value(u8),
    List(Vec<ListItem>)
}

// rules for comparing lists
impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // compare values
            (ListItem::Value(a), ListItem::Value(b)) => {
                a.cmp(b)
            },
            // compare lists of values
            (ListItem::List(list_a), ListItem::List(list_b)) => {
                if list_b.is_empty() {
                    if list_a.is_empty() {
                        return Ordering::Equal;
                    } else {
                        return Ordering::Greater;
                    }
                }

                for (i, a) in list_a.iter().enumerate() {
                    if i > list_b.len() - 1 || a > &list_b[i] {
                        return Ordering::Greater;
                    }

                    if a < &list_b[i] {
                        return Ordering::Less;
                    }

                    // last item in list_a
                    if i == list_a.len() - 1 {
                        if i == list_b.len() - 1 {
                            return Ordering::Equal;
                        } else {
                            return Ordering::Less;
                        }
                    }
                }

                // List A is empty, so inputs are in right order
                Ordering::Less
            },
            // compare lists of values against values
            (list_a, ListItem::Value(b)) => {
                let list_b = ListItem::List(vec![ListItem::Value(*b)]);
                list_a.cmp(&list_b)
            },
            // compare values against lists of values
            (ListItem::Value(a), list_b) => {
                let list_a = ListItem::List(vec![ListItem::Value(*a)]);
                list_a.cmp(list_b)
            }
        }

    }
}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ListItem {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

// recursive parser
fn parse_list(input: &str) -> Result<ListItem, ()> {
    let mut depth = 0;
    let mut elems: Vec<&str> = vec![];
    let mut offset = 0;

    if !input.starts_with('[') && input.ends_with(']') {
        return Err(());
    }

    // discard outer brackets
    let input = &input[1..input.len() - 1];

    if input.is_empty() {
        return Ok(ListItem::List(vec![]));
    }

    let mut list: Vec<ListItem> = vec![]; 

    for (i, c) in input.chars().enumerate() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            _ => ()
        }
        
        if c == ',' && depth == 0 {
            elems.push(&input[offset..i]);
            offset = i + 1;
        }
    }

    if offset == 0 {
        elems.push(input);
    } else {
        elems.push(&input[offset..]);
    }

    for elem in elems {
        if elem.starts_with('[') {
            // recurse if element contains a list
            list.push(parse_list(elem).unwrap());
        } else {
            // base case
            let value = ListItem::Value(elem.parse::<u8>().unwrap());
            list.push(value);
        }
    }

    Ok(ListItem::List(list))
}

pub fn part1(input: &str) -> usize {
    let packets = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|packet| parse_list(packet).unwrap())
        .collect::<Vec<_>>();

    let pairs = packets.chunks(2);

    pairs
        .enumerate()
        .map(|(i, pair)| {
            if pair[0] < pair[1] {
                i + 1
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut packets = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|packet| parse_list(packet).unwrap())
        .collect::<Vec<_>>();

    packets.push(parse_list("[[2]]").unwrap());
    packets.push(parse_list("[[6]]").unwrap());

    packets.sort();

    let dividers = [
        parse_list("[[2]]").unwrap(),
        parse_list("[[6]]").unwrap()
    ];

    packets
        .iter()
        .enumerate()
        .filter_map(|(i, packet)| {
            if dividers.contains(packet) {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "[1,1,3,1,1]\n\
                              [1,1,5,1,1]\n\n\
                              [[1],[2,3,4]]\n\
                              [[1],4]\n\n\
                              [9]\n\
                              [[8,7,6]]\n\n\
                              [[4,4],4,4]\n\
                              [[4,4],4,4,4]\n\n\
                              [7,7,7,7]\n\
                              [7,7,7]\n\n\
                              []\n\
                              [3]\n\n\
                              [[[]]]\n\
                              [[]]\n\n\
                              [1,[2,[3,[4,[5,6,7]]]],8,9]\n\n\
                              [1,[2,[3,[4,[5,6,0]]]],8,9]\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 140);
    }
}

