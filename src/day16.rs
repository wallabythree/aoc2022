use core::cmp::Ordering;
use std::collections::VecDeque;

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
    edges: Vec<(usize, usize)>
}

impl Node {
    fn from(line: &str) -> (Self, (usize, usize)) {
        let mut parts = line
            .split([' ', '=', ';', ',']);

        let code = parts.nth(1).unwrap().as_bytes();
        let (y, x) = (
            (code[0] - b'A') as usize,
            (code[1] - b'A') as usize
        );

        let value = parts.nth(3).unwrap().parse().unwrap();
        
        let mut edges: Vec<(usize, usize)>  = vec![];

        for edge in parts.skip(5) {
            if edge.is_empty() {
                continue;
            }

            let code = edge.as_bytes();

            let (y, x) = (
                (code[0] - b'A') as usize,
                (code[1] - b'A') as usize
            );

            edges.push((x, y));
        }

        (Self { value, open: false, edges }, (x, y))
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

#[derive(Debug)]
struct Map {
    nodes: [[Option<Node>; 26]; 26]
}

impl Map {
    fn bfs(
        &self,
        s: (usize, usize),
        goal: (usize, usize)
    ) -> Result<usize, ()> {

        let mut visited = [[false; 26]; 26];
        visited[s.1][s.0] = true;

        let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
        queue.push_back(((s.1, s.0), 0));

        while let Some(node) = queue.pop_front() {
            if node.0 == goal {
                //println!("Found! {:?}", node.0);
                return Ok(node.1);
            }

            for edge in self.nodes[node.0.1][node.0.0].clone().unwrap().edges {
                if !visited[edge.1][edge.0] && edge != node.0 {
                    queue.push_back(((edge.0, edge.1), node.1 + 1));
                    visited[edge.1][edge.0] = true;
                }
            }
        }

        Err(())
    }
}

pub fn part1(input: &str) -> usize {
    let nodes: [[Option<Node>; 26]; 26] = Default::default();
    let mut map = Map { nodes };


    let mut start: Option<(usize, usize)> = None;

    input
        .trim()
        .lines()
        .map(Node::from)
        .for_each(|node_result| {
            let (Node, (x, y)) = node_result;

            if start.is_none() {
                start = Some((x, y));
            }

            map.nodes[y][x] = Some(Node);
        });

    println!("{:?}", map);

    let mut pos = start.unwrap();
    let mut pressure_released = 0;
    let mut target: Option<(usize, usize)> = None;

    for minute in 1..31 {
        
        pressure_released += map
            .nodes
            .iter()
            .map(|row| {
                row
                    .iter()
                    .filter_map(|valve_opt| {
                        if let Some(valve) = valve_opt {
                            if valve.open {
                                Some(valve.value)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();

            if let Some(t) = target {
                if pos == t {
                    // open valve
                    if let Some(target_node) = &mut map.nodes[t.1][t.0] {
                        target_node.open = true;
                        target = None;
                    }
                } else {
                    // move towards target
                    let mut next = (0, 0);
                    let mut distance = usize::MAX;

                    for edge in map.nodes[pos.1][pos.0].clone().unwrap().edges {
                        let edge_distance = map.bfs(edge, t).unwrap();

                        if edge_distance < distance {
                            next = edge;
                            distance = edge_distance;
                        }
                    }

                    pos = next;
                }

            } else {
                let mut target_pos: Option<(usize, usize)> = None;
                let mut top_score = 0.0;

                for y in 0..26 {
                    for x in 0..26 {
                        if (x, y) == pos {
                            continue;
                        }

                        if let Some(node) = &map.nodes[y][x] {
                            if !node.open {
                                let distance = map.bfs(pos, (x,y)).unwrap();
                                let score = node.value as f64 / distance as f64;

                                if score >= top_score {
                                    top_score = score;
                                    target_pos = Some((x, y));
                                }
                            }
                        }
                    }
                }

                target = target_pos;
                println!("{:?}", target_pos);
            }

    }

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

