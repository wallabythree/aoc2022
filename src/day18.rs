use std::collections::VecDeque;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
struct Vertex {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug,Clone,Copy)]
struct Side {
    vertices: [Vertex; 4],
}

impl PartialEq for Side {
    fn eq(&self, other: &Self) -> bool {
        self.vertices.iter().all(|vertex| other.vertices.contains(vertex))
    }
}

impl Eq for Side {}

#[derive(Debug,Clone,Copy,PartialEq)]
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

fn bfs(cubes: &[Cube]) -> usize {
    let root = Cube { x: 0, y: 0, z: 0 };
    let mut visited = [[[false; 30]; 30]; 30];

    let mut queue: VecDeque<Cube> = VecDeque::new();
    queue.push_back(root);

    let mut side_count = 0;

    while let Some(cube) = queue.pop_front() {
        for side in cube.sides() {
            side_count += cubes.iter().filter(|other_cube| other_cube.sides().contains(&side)).count();
        }

        let mut in_front = cube;
        in_front.x += 1;

        let mut behind = cube;
        behind.x -= 1;

        let mut left = cube;
        left.y -= 1;

        let mut right = cube;
        right.y += 1;

        let mut above = cube;
        above.z += 1;

        let mut below = cube;
        below.z -= 1;

        let adjacent_cubes = [in_front, behind, left, right, above, below];

        for next in adjacent_cubes {
            if next.x >= 0 && next.y >= 0 && next.z >= 0
               && next.x < 30 && next.y < 30 && next.z < 30
               && !cubes.contains(&next)
               && !visited[next.z as usize][next.y as usize][next.x as usize] {
                   visited[next.z as usize][next.y as usize][next.x as usize] = true;
                   queue.push_back(next);
               }
        }
    }

    side_count
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

    let all_sides: Vec<Side> = cubes
        .iter()
        .flat_map(|cube| cube.sides())
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

                    for x in -30..0 {
                        let possible_neighbour = Vertex { x: v.x + x, y: v.y, z: v.z };
                        if all_vertices.contains(&possible_neighbour) {
                            neighbour_count += 1;
                            break;
                        }
                    }

                    for x in 1..30 {
                        let possible_neighbour = Vertex { x: v.x + x, y: v.y, z: v.z };
                        if all_vertices.contains(&possible_neighbour) {
                            neighbour_count += 1;
                            break;
                        }
                    }

                    for y in -30..0 {
                        let possible_neighbour = Vertex { x: v.x, y: v.y + y, z: v.z };
                        if all_vertices.contains(&possible_neighbour) {
                            neighbour_count += 1;
                            break;
                        }
                    }

                    for y in 1..30 {
                        let possible_neighbour = Vertex { x: v.x, y: v.y + y, z: v.z };
                        if all_vertices.contains(&possible_neighbour) {
                            neighbour_count += 1;
                            break;
                        }
                    }

                    for z in -30..0 {
                        let possible_neighbour = Vertex { x: v.x, y: v.y, z: v.z + z};
                        if all_vertices.contains(&possible_neighbour) {
                            neighbour_count += 1;
                            break;
                        }
                    }

                    for z in 1..30 {
                        let possible_neighbour = Vertex { x: v.x, y: v.y, z: v.z + z };
                        if all_vertices.contains(&possible_neighbour) {
                            neighbour_count += 1;
                            break;
                        }
                    }

                    // check if candidate has opposite side
                    if neighbour_count == 6 {
                        let mut dir_x = 0;
                        let mut dir_y = 0;
                        let mut dir_z = 0;

                        match side.face {
                            Face::Front => dir_x = 1,
                            Face::Rear => dir_x = -1,
                            Face::Left => dir_y = -1,
                            Face::Right => dir_y = 1,
                            Face::Top => dir_z = 1,
                            Face::Bottom => dir_z = -1,
                        }

                        for i in 1..30i64 {
                            let opposite = Side {
                                vertices: [
                                    Vertex { 
                                        x: side.vertices[0].x + dir_x * i,
                                        y: side.vertices[0].y + dir_y * i,
                                        z: side.vertices[0].z + dir_z * i
                                    },
                                    Vertex { 
                                        x: side.vertices[1].x + dir_x * i,
                                        y: side.vertices[1].y + dir_y * i,
                                        z: side.vertices[1].z + dir_z * i
                                    },
                                    Vertex { 
                                        x: side.vertices[2].x + dir_x * i,
                                        y: side.vertices[2].y + dir_y * i,
                                        z: side.vertices[2].z + dir_z * i
                                    },
                                    Vertex { 
                                        x: side.vertices[3].x + dir_x * i,
                                        y: side.vertices[3].y + dir_y * i,
                                        z: side.vertices[3].z + dir_z * i
                                    },
                                ],
                                face: side.face,
                            };

                            /*
                            for j in 0..4 {
                                opposite.vertices[j].x += dir_x * i;
                                opposite.vertices[j].y += dir_y * i;
                                opposite.vertices[j].z += dir_z * i;
                            }
                            */
                            
                            if all_sides.contains(&opposite) {
                                return true;
                            }
                        }
                    }
                    
                    false
                })
        })
        .collect();


        println!("exposed sides: {}, air pocket sides: {}", exposed_sides.len(), air_pockets.len());
        exposed_sides.len() - air_pockets.len()
            */
    bfs(&cubes)
}

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

