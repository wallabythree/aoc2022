#[derive(Debug,Clone,Copy)]
struct World {
    costs: [[i64; 3]; 4],
    robots: [i64; 4],
    resources: [i64; 4],
    time: i64,
}

impl World {
    fn new(costs: [[i64; 3]; 4]) -> Self {
        Self { costs, robots: [1, 0, 0, 0], resources: [0; 4], time: 0 }
    }

    fn mine_resources(&mut self) {
        for (i, p) in self.robots.iter().enumerate() {
            self.resources[i] += p;
        }
    }

    fn tick(&mut self, new_robot: Option<usize>) -> i64 {
        //if self.time > 17 && self.robots[3] < 1 {
        //    return 0;
        //}

        if ((24 - self.time) * self.robots[0] + self.resources[0] < self.costs[3][0]) && self.robots[3] == 0 {
            return 0;
        }

        // base case
        if self.time >= 24 {
            //if self.resources[3] >= 12 {
            //    println!("tick! {:?}", self);
            //}

            return self.resources[3];
        }

        self.mine_resources();
        self.time += 1;
        
        if let Some(i) = new_robot {
            self.robots[i] += 1;
        }


        if self.time < 23 {
            let mut geodes_from_worlds = [0, 0, 0, 0, 0];

            for (i, robot_costs) in self.costs.iter().enumerate() {
                if self.resources[0] < robot_costs[0]
                   || self.resources[1] < robot_costs[1]
                   || self.resources[2] < robot_costs[2] {
                       continue;
                }

                let mut new_world = self.to_owned();

                new_world.resources[0] -= robot_costs[0];
                new_world.resources[1] -= robot_costs[1];
                new_world.resources[2] -= robot_costs[2];

                geodes_from_worlds[i] = new_world.tick(Some(i));

            }
            geodes_from_worlds[4] = self.clone().tick(None);

            return *geodes_from_worlds.iter().max().unwrap();
        } else {
            self.clone().tick(None)
        }
    }
}

#[derive(Debug)]
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

    fn simulate(&self) -> i64 {
        let mut world = World::new(self.costs);

        let geodes = world.tick(None);

        //println!("{:?}", world);

        geodes
    }
}

pub fn part1(input: &str) -> usize {
    let blueprints: Vec<_> = input
        .lines()
        .map(Blueprint::from)
        .collect();

    let mut quality_levels = 0;

    for blueprint in blueprints {
        let geodes = blueprint.simulate() as usize;
        let quality_level = blueprint.id * geodes;

        quality_levels += quality_level;

        println!("{:?} geodes: {} quality level: {}", blueprint, geodes, quality_level);
    }

    quality_levels
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 33);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), TEST_INPUT.len());
    }
}

