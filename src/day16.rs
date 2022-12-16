use core::cmp::Ordering;
use std::collections::HashMap;

/* DOT langauge representation of graph
digraph G {
    concentrate=true;
    
    AA -> DD
    AA -> II
    AA -> B_B
    
    B_B -> CC
    B_B -> AA
    
    CC -> DD
    CC -> B_B
    
    DD -> CC
    DD -> AA
    DD -> EE
    
    EE -> FF
    EE -> DD
    
    FF -> EE
    FF -> GG
    
    GG -> FF
    GG -> HH
    
    HH -> GG
    
    II -> AA
    II -> JJ
    
    JJ -> II
}
*/

// algorithm:
// 0. start at node AA
// 1. find optimal target node (node_value / node_distance)
// 2. move to target node
// 3. open target node
// 4. repeat steps 1-3 until time runs out or all nodes are open

#[derive(Debug,Default,Clone,Eq,PartialEq)]
struct Node {
    value: usize,
    open: bool,
    edges: Vec<String>
}

impl Node {
    fn from(line: &str) -> (Self, &str) {
        let mut parts = line
            .split([' ', '=', ';', ',']);

        let key = parts.nth(1).unwrap();

        let value = parts.nth(3).unwrap().parse().unwrap();
        
        let mut edges = vec![];

        for edge in parts.skip(5) {
            if edge.is_empty() {
                continue;
            }

            edges.push(edge.to_owned());
        }

        (Self { value, open: false, edges }, key)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: &str) -> usize {
    let mut graph = HashMap::new();

    let mut start: Option<&str> = None;

    input
        .trim()
        .lines()
        .map(Node::from)
        .for_each(|node_result| {
            let (node, key) = node_result;

            if start.is_none() {
                start = Some(key);
            }

            graph.insert(key, node);
        });

    println!("{:?}", graph);

    let mut pos = start.unwrap();
    let mut pressure_released = 0;
    let mut target: Option<(usize, usize)> = None;


    pressure_released
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1651);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), TEST_INPUT.len());
    }
}

