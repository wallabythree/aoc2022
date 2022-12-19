use std::thread;

#[derive(Clone,Copy)]
struct World {
    costs: [[i64; 3]; 4],
    robots: [i64; 4],
    resources: [i64; 4],
    time: usize,
    queued: Option<usize>,
}

impl World {
    fn new(costs: [[i64; 3]; 4], time: usize) -> Self {
        Self {
            costs,
            robots: [1, 0, 0, 0],
            resources: [0; 4],
            time,
            queued: None
        }
    }

    fn mine_resources(&mut self) {
        for (i, p) in self.robots.iter().enumerate() {
            self.resources[i] += p;
        }
    }

    fn can_afford(&self, kind: usize) -> bool {
        self.resources[0] >= self.costs[kind][0]
        && self.resources[1] >= self.costs[kind][1]
        && self.resources[2] >= self.costs[kind][2]
    }

    fn build(&mut self, kind: usize) {
        self.resources[0] -= self.costs[kind][0];
        self.resources[1] -= self.costs[kind][1];
        self.resources[2] -= self.costs[kind][2];
        self.queued = Some(kind);
    }

    fn tick(&mut self, max: &mut [i64]) -> i64 {

        // base case
        if self.time == 0 {
            return self.resources[3];
        }

        self.mine_resources();
        self.time -= 1;

        // memoization
        if max[self.time] > self.resources[3] {
            return 0;
        }
        max[self.time] = self.resources[3];
        
        if let Some(i) = self.queued {
            self.robots[i] += 1;
            self.queued = None;
        }

        if self.time > 1 {
            if self.can_afford(3) {
                self.build(3);
            } else if self.time > 3 {
                let mut geodes_from_worlds = [0, 0, 0, 0];

                for i in 0..3 {
                    if i == 0 && self.costs[i][0] >= self.time as i64 + 1
                       || i == 1 && self.time <= 5 {
                        continue;
                    }

                    if !self.can_afford(i) {
                        continue;
                    }

                    let mut new_world = self.to_owned();
                    new_world.build(i);
                    
                    geodes_from_worlds[i] = new_world.tick(max);
                }

                geodes_from_worlds[3] = self.tick(max);

                return *geodes_from_worlds.iter().max().unwrap();
            }
        }

        return self.tick(max);
    }
}

struct Blueprint {
    id: usize,
    costs: [[i64; 3]; 4],
}

impl Blueprint {
    fn from(input: &str) -> Self {
        let parts: Vec<_> = input.split([' ', ':']).collect();
        let id = parts[1].parse().unwrap();

        let costs = [
            [parts[7].parse().unwrap(), 0, 0],
            [parts[13].parse().unwrap(), 0, 0],
            [parts[19].parse().unwrap(), parts[22].parse().unwrap(), 0],
            [parts[28].parse().unwrap(), 0, parts[31].parse().unwrap()],
        ];

        Self { id, costs }
    }

    fn simulate(&self, time: usize) -> i64 {
        let mut world = World::new(self.costs, time);
        let mut max = vec![0i64; time];

        world.tick(&mut max)
    }
}

pub fn part1(input: &str) -> usize {
    let blueprints: Vec<_> = input
        .lines()
        .map(Blueprint::from)
        .collect();

    let mut handles = vec![];

    for blueprint in blueprints {
        let handle = thread::spawn(move || {
            let geodes = blueprint.simulate(24) as usize;
            blueprint.id * geodes
        });

        handles.push(handle);
    }

    let mut quality_levels = 0;

    for handle in handles {
        quality_levels += handle.join().unwrap();
    }

    quality_levels
}

pub fn part2(input: &str) -> usize {
    let mut blueprints: Vec<_> = input
        .lines()
        .map(Blueprint::from)
        .collect();

    blueprints.truncate(3);

    let mut handles = vec![];

    for blueprint in blueprints {
        let handle = thread::spawn(move || {
            blueprint.simulate(32) as usize
        });

        handles.push(handle);
    }

    let mut geode_product = 1;

    for handle in handles {
        geode_product *= handle.join().unwrap();
    }

    geode_product
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each \
                              clay robot costs 2 ore. Each obsidian robot \
                              costs 3 ore and 14 clay. Each geode robot costs \
                              2 ore and 7 obsidian.\n\
                              Blueprint 2: Each ore robot costs 2 ore. Each \
                              clay robot costs 3 ore. Each obsidian robot \
                              costs 3 ore and 8 clay. Each geode robot costs \
                              3 ore and 12 obsidian.\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 33);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 56);
    }
}

