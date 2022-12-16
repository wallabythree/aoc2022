use core::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
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

#[derive(Debug,Default,Clone,Eq,PartialEq,Hash)]
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

fn bfs(
    graph: &HashMap<&str, Node>,
    root: &str,
    goal: &str
) -> Result<usize, ()> {
    let mut queue: VecDeque<(&str, usize)> = VecDeque::new();

    queue.push_back((root, 0));

    while let Some(node) = queue.pop_front() {
        if node.0 == goal {
            return Ok(node.1);
        }

        for edge_key in graph.get(node.0).unwrap().edges.iter() {
            if graph.get(edge_key as &str).is_some() {
                queue.push_back((edge_key, node.1 + 1));
            }
        }
    }

    Err(())
}

fn max_flow(
    graph: &HashMap<&str, Node>,
    distances: &HashMap<&str, HashMap<&str, usize>>,
    start: &str,
    to_visit: HashSet<&str>,
    minutes: usize,
    pressure: usize,
    acc: usize,
    max: &mut usize
) {
     // base case
    if minutes >= 30 || to_visit.is_empty() {
        let  total = acc + pressure * (30 - minutes);
        if total > *max {
            *max = total;
        }

        return;
    }

    // recursive case
    for dst in &to_visit {
        let node = &graph.get(dst).unwrap();

        let mut remaining = to_visit.clone();
        remaining.remove(dst);

        let minutes_to_dst = distances.get(start).unwrap().get(dst).unwrap();
        let new_minutes = (minutes_to_dst + 1).min(30 - minutes);

        max_flow(
            graph,
            distances,
            dst,
            remaining,
            minutes + new_minutes,
            pressure + node.value,
            acc + new_minutes * pressure,
            max
        )
    }
}

fn max_flow_with_elephant(
    graph: &HashMap<&str, Node>,
    distances: &HashMap<&str, HashMap<&str, usize>>,
    me: &str,
    elephant: &str,
    to_visit: BTreeSet<&str>,
    minutes_me: usize,
    minutes_elephant: usize,
    pressure_me: usize,
    pressure_elephant: usize,
    mut ticks: [usize; 26],
    max: &mut usize
) {
     // base case
    if (minutes_me >= 26 && minutes_elephant >= 26) || to_visit.is_empty() {

        let mut new_ticks = ticks.clone();
        for i in minutes_me..26 {
            ticks[i] += pressure_me;
        }
        for i in minutes_elephant..26 {
            ticks[i] += pressure_elephant;
        }

        let total = new_ticks.iter().sum();

        if total > *max {
            *max = total;
            println!("ticks: {:?}", ticks);
            println!("new max: {}", *max);
        }

        return;
    }

    if to_visit.len() == 1 {
        let dst = to_visit.iter().next().unwrap();
        let node = &graph.get(dst).unwrap();

        let mut remaining = to_visit.clone();
        remaining.remove(dst);

        let minutes_to_dst_me = distances
            .get(me)
            .unwrap()
            .get(dst)
            .unwrap();
        let new_minutes_me = (minutes_to_dst_me + 1).min(26 - minutes_me);

        let mut me_ticks = ticks.clone();
        for i in minutes_me..(minutes_me + new_minutes_me) {
            me_ticks[i] += pressure_me + node.value;
        }

        max_flow_with_elephant(
            graph,
            distances,
            dst,
            elephant,
            remaining.clone(),
            minutes_me + new_minutes_me,
            minutes_elephant,
            pressure_me + node.value,
            pressure_elephant,
            me_ticks,
            max
        );

        let minutes_to_dst_elephant = distances
            .get(elephant)
            .unwrap()
            .get(dst)
            .unwrap();

        let new_minutes_elephant = (minutes_to_dst_elephant + 1).min(26 - minutes_elephant);

        let mut elephant_ticks = ticks.clone();
        for i in minutes_elephant..(minutes_elephant + new_minutes_elephant) {
            elephant_ticks[i] += node.value;
        }

        max_flow_with_elephant(
            graph,
            distances,
            me,
            dst,
            remaining,
            minutes_me,
            minutes_elephant + new_minutes_elephant,
            pressure_me,
            pressure_elephant + node.value,
            elephant_ticks,
            max
        );
    }

    // recursive case
    for dst_me in to_visit.iter().take(to_visit.len()) {
        let node_me = &graph.get(dst_me).unwrap();

        let mut remaining = to_visit.clone();
        remaining.remove(dst_me);

        let minutes_to_dst_me = distances
            .get(me)
            .unwrap()
            .get(dst_me)
            .unwrap();
        let new_minutes_me = (minutes_to_dst_me + 1).min(26 - minutes_me);

        let mut new_ticks = ticks.clone();
        for i in minutes_me..(minutes_me + new_minutes_me) {
            ticks[i] += pressure_me;
        }

        for dst_elephant in remaining.iter() {
            if dst_me == dst_elephant {
                continue;
            }

            let node_elephant = &graph.get(dst_elephant).unwrap();

            let mut remaining = remaining.clone();
            remaining.remove(dst_elephant);

            let minutes_to_dst_elephant = distances
                .get(elephant)
                .unwrap()
                .get(dst_elephant)
                .unwrap();

            let new_minutes_elephant = (minutes_to_dst_elephant + 1).min(26 - minutes_elephant);

            let mut new_ticks = new_ticks.clone();
            for j in minutes_elephant..(minutes_elephant + new_minutes_elephant) {
                ticks[j] += pressure_elephant;
            }


            max_flow_with_elephant(
                graph,
                distances,
                dst_me,
                dst_elephant,
                remaining,
                minutes_me + new_minutes_me,
                minutes_elephant + new_minutes_elephant,
                pressure_me + node_me.value,
                pressure_elephant + node_elephant.value,
                new_ticks,
                max
            )
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut graph = HashMap::new();

    input
        .trim()
        .lines()
        .map(Node::from)
        .for_each(|node_result| {
            let (node, key) = node_result;

            graph.insert(key, node);
        });

    let to_visit = graph
        .iter()
        .filter_map(|(key, node)| {
            if node.value > 0 {
                Some(*key)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    let distances = graph
        .keys()
        .map(|key| {
            let key_distances = graph
                .keys()
                .into_iter()
                .map(|other_key| (*other_key, bfs(&graph, key, other_key).unwrap()))
                .collect::<HashMap<&str, usize>>();
            (*key, key_distances)
        })
        .collect::<HashMap<&str, HashMap<&str, usize>>>();

    println!("{:?}", to_visit);
    println!("nodes with valves: {}", to_visit.len());

    let mut max = 0;

    max_flow(
        &graph,
        &distances,
        "AA",
        to_visit,
        0,
        0,
        0,
        &mut max
    );

    max
}

pub fn part2(input: &str) -> usize {
    let mut graph = HashMap::new();

    input
        .trim()
        .lines()
        .map(Node::from)
        .for_each(|node_result| {
            let (node, key) = node_result;

            graph.insert(key, node);
        });

    let to_visit = graph
        .iter()
        .filter_map(|(key, node)| {
            if node.value > 0 {
                Some(*key)
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>();

    let distances = graph
        .keys()
        .map(|key| {
            let key_distances = graph
                .keys()
                .into_iter()
                .map(|other_key| (*other_key, bfs(&graph, key, other_key).unwrap()))
                .collect::<HashMap<&str, usize>>();
            (*key, key_distances)
        })
        .collect::<HashMap<&str, HashMap<&str, usize>>>();

    println!("{:?}", to_visit);
    println!("nodes with valves: {}", to_visit.len());

    let mut max = 0;

    max_flow_with_elephant(
        &graph,
        &distances,
        "AA",
        "AA",
        to_visit,
        0,
        0,
        0,
        0,
        [0; 26],
        &mut max
    );

    max
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n\
                              Valve BB has flow rate=13; tunnels lead to valves CC, AA\n\
                              Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
                              Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n\
                              Valve EE has flow rate=3; tunnels lead to valves FF, DD\n\
                              Valve FF has flow rate=0; tunnels lead to valves EE, GG\n\
                              Valve GG has flow rate=0; tunnels lead to valves FF, HH\n\
                              Valve HH has flow rate=22; tunnel leads to valve GG\n\
                              Valve II has flow rate=0; tunnels lead to valves AA, JJ\n\
                              Valve JJ has flow rate=21; tunnel leads to valve II\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1651);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1707);
    }
}

