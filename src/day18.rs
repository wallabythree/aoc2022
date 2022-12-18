#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash,PartialOrd,Ord)]
struct Vertex {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug,Clone,Copy,PartialOrd,Ord)]
struct Side {
    vertices: [Vertex; 4]
}

impl PartialEq for Side {
    fn eq(&self, other: &Self) -> bool {
        self.vertices.iter().all(|vertex| other.vertices.contains(vertex))
    }
}

impl Eq for Side {}

#[derive(Debug,PartialEq)]
struct Cube {
    x: i64,
    y: i64,
    z: i64,
}

impl Cube {
    fn from(input: &str) -> Self {
        let pos: Vec<i64> = input
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect();

        Self { x: pos[0], y: pos[1], z: pos[2] }
    }

    fn vertices(&self) -> [Vertex; 8] {
        let (x, y, z) = (self.x, self.y, self.z);

        [
            Vertex { x, y, z },
            Vertex { x: x + 1, y,  z },
            Vertex { x: x + 1, y: y + 1,  z },
            Vertex { x, y: y + 1,  z },
            Vertex { x, y, z: z + 1 },
            Vertex { x: x + 1, y,  z: z + 1 },
            Vertex { x: x + 1, y: y + 1,  z: z + 1 },
            Vertex { x, y: y + 1,  z: z + 1 }
        ]
    }

    fn sides(&self) -> [Side; 6] {
        let v = self.vertices();

        [
            // Bottom
            Side { vertices: [ v[0], v[1], v[2], v[3] ] },
            // Top
            Side { vertices: [ v[4], v[5], v[6], v[7] ] },
            // Left
            Side { vertices: [ v[0], v[1], v[5], v[4] ] },
            // Right
            Side { vertices: [ v[2], v[3], v[7], v[6] ] },
            // Front
            Side { vertices: [ v[1], v[2], v[6], v[5] ] },
            // Rear
            Side { vertices: [ v[0], v[4], v[7], v[3] ] },
        ]
    }
}

pub fn part1(input: &str) -> usize {
    let cubes: Vec<Cube> = input
        .trim()
        .split('\n')
        .map(Cube::from)
        .collect();

    cubes
        .iter()
        .map(|cube| {
             cube
                .sides()
                .iter()
                .filter(|side| {
                    cubes
                        .iter()
                        .all(|other_cube|{
                            other_cube == cube
                            || !other_cube.sides().contains(side)
                        })
                }).count()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let cubes: Vec<Cube> = input
        .trim()
        .split('\n')
        .map(Cube::from)
        .collect();

    let sides: Vec<Side> = cubes
        .iter()
        .flat_map(|cube| cube.sides())
        .collect();

    // front -> rear
    let mut x_face_counter = [[false; 50]; 50];

    for x in (0..50).rev() {
        for y in 0..50 {
            for z in 0..50 {
                let side = Side { vertices: [
                    Vertex { x, y, z },
                    Vertex { x, y: y + 1, z },
                    Vertex { x, y: y + 1, z: z + 1 },
                    Vertex { x, y, z: z + 1 },
                ] };

                if sides.contains(&side) {
                    x_face_counter[z as usize][y as usize] = true;
                }
            }
        }
    }

    let mut y_face_counter = [[false; 50]; 50];
    // top -> bottom
    for y in (0..50).rev() {
        for x in 0..50 {
            for z in 0..50 {
                let side = Side { vertices: [
                    Vertex { x: x + 1, y, z },
                    Vertex { x, y, z },
                    Vertex { x, y, z: z + 1},
                    Vertex { x: x + 1, y, z: z + 1 },
                ] };

                if sides.contains(&side) {
                    y_face_counter[z as usize][x as usize] = true;
                }
            }
        }
    }

    let mut z_face_counter = [[false; 50]; 50];
    // top -> bottom
    for z in (0..50).rev() {
        for x in 0..50 {
            for y in 0..50 {
                let side = Side { vertices: [
                    Vertex { x, y, z },
                    Vertex { x: x + 1, y, z },
                    Vertex { x: x + 1, y: y + 1, z },
                    Vertex { x, y: y + 1, z },
                ] };

                if sides.contains(&side) {
                    z_face_counter[y as usize][x as usize] = true;
                }
            }
        }
    }

    x_face_counter
        .map(|row| {
            row
                .iter()
                .filter(|square| **square)
                .count()
        })
        .iter()
        .sum::<usize>() * 2
    +
    y_face_counter
        .map(|row| {
            row
                .iter()
                .filter(|square| **square)
                .count()
        })
        .iter()
        .sum::<usize>() * 2
    + 
    z_face_counter
        .map(|row| {
            row
                .iter()
                .filter(|square| **square)
                .count()
        })
        .iter()
        .sum::<usize>() * 2
}

    /*
    let exposed_sides: Vec<Side> = cubes
        .iter()
        .flat_map(|cube| {
             cube
                .sides()
                .iter()
                .filter(|side| {
                    cubes
                        .iter()
                        .all(|other_cube|{
                            other_cube == cube
                            || !other_cube.sides().contains(side)
                        })
                })
                .copied()
                .collect::<Vec<Side>>()
        })
        .collect();

    let all_vertices: Vec<Vertex> = cubes
        .iter()
        .flat_map(|cube| {
            cube
                .sides()
                .iter()
                .flat_map(|side| side.vertices).collect::<Vec<Vertex>>()
        })
        .collect();

    let air_pockets: Vec<_> = exposed_sides
        .iter()
        .filter(|side| {
            side
                .vertices
                .iter()
                .all(|v| {
                    let mut neighbour_count = 0;

                    for x in -25..0 {
                        let possible_neighbour = Vertex { x: v.x + x, y: v.y, z: v.z };
                        if all_vertices.contains(&possible_neighbour) {
                            neighbour_count += 1;
                            break;
                        }
                    }

                    for x in 1..25 {
                        let possible_neighbour = Vertex { x: v.x + x, y: v.y, z: v.z };
                        if all_vertices.contains(&possible_neighbour) {
                            neighbour_count += 1;
                            break;
                        }
                    }

                    for y in -25..0 {
                        let possible_neighbour = Vertex { x: v.x, y: v.y + y, z: v.z };
                        if all_vertices.contains(&possible_neighbour) {
                            neighbour_count += 1;
                            break;
                        }
                    }

                    for y in 1..25 {
                        let possible_neighbour = Vertex { x: v.x, y: v.y + y, z: v.z };
                        if all_vertices.contains(&possible_neighbour) {
                            neighbour_count += 1;
                            break;
                        }
                    }

                    for z in -25..0 {
                        let possible_neighbour = Vertex { x: v.x, y: v.y, z: v.z + z};
                        if all_vertices.contains(&possible_neighbour) {
                            neighbour_count += 1;
                            break;
                        }
                    }

                    for z in 1..25 {
                        let possible_neighbour = Vertex { x: v.x, y: v.y, z: v.z + z };
                        if all_vertices.contains(&possible_neighbour) {
                            neighbour_count += 1;
                            break;
                        }
                    }

                    neighbour_count == 6
                })
        })
        .collect();


        println!("exposed sides: {}, air pocket sides: {}", exposed_sides.len(), air_pockets.len());
        exposed_sides.len() - air_pockets.len()
        */

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "2,2,2\n\
                              1,2,2\n\
                              3,2,2\n\
                              2,1,2\n\
                              2,3,2\n\
                              2,2,1\n\
                              2,2,3\n\
                              2,2,4\n\
                              2,2,6\n\
                              1,2,5\n\
                              3,2,5\n\
                              2,1,5\n\
                              2,3,5\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 64);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 58);
    }
}

