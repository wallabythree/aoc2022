use std::collections::VecDeque;

#[derive(Clone,Copy)]
struct Node {
    x: usize,
    y: usize,
    d: usize,
    val: u8
}

struct Matrix<'a> {
    rows: Vec<&'a [u8]>
}

impl<'a> Matrix<'a> {
    fn new(input: &'a str) -> Self {
        let rows: Vec<_> = input
            .trim()
            .as_bytes()
            .split(|&b| b == b'\n')
            .collect();

        Self { rows }
    }

    fn rows(&self) -> usize {
        self.rows.len()
    }

    fn cols(&self) -> usize {
        if let Some(row) = self.rows.first() {
            return row.len();
        }

        0
    }

    fn bfs<F,G>(
        &self,
        s: (usize, usize),
        is_goal: F,
        is_valid: G
    ) -> Result<usize, ()>
    where F: Fn(Node) -> bool, G: Fn(Node, Node) -> bool {

        let root = Node { x: s.0, y: s.1, d: 0, val: self.rows[s.1][s.0] };

        let mut visited = vec![vec![false; self.cols()]; self.rows()];
        visited[root.y][root.x] = true;

        let mut queue: VecDeque<Node> = VecDeque::new();
        queue.push_back(root);

        while let Some(node) = queue.pop_front() {
            if is_goal(node) {
                return Ok(node.d);
            }

            if node.x > 0 {
                let left = Node {
                    x: node.x - 1,
                    y: node.y,
                    d: node.d + 1,
                    val: self.rows[node.y][node.x - 1]
                };

                if !visited[left.y][left.x] && is_valid(node, left) {
                    queue.push_back(left);
                    visited[left.y][left.x] = true;
                }
            }

            if node.x < self.cols() - 1 {
                let right = Node {
                    x: node.x + 1,
                    y: node.y,
                    d: node.d + 1,
                    val: self.rows[node.y][node.x + 1],
                };

                if !visited[right.y][right.x] && is_valid(node, right) {
                    queue.push_back(right);
                    visited[right.y][right.x] = true;
                }
            }

            if node.y > 0 {
                let up = Node {
                    x: node.x,
                    y: node.y - 1,
                    d: node.d + 1,
                    val: self.rows[node.y - 1][node.x],
                };

                if !visited[up.y][up.x] && is_valid(node, up) {
                    queue.push_back(up);
                    visited[up.y][up.x] = true;
                }
            }

            if node.y < self.rows() - 1 {
                let down = Node {
                    x: node.x,
                    y: node.y + 1,
                    d: node.d + 1,
                    val: self.rows[node.y + 1][node.x],
                };

                if !visited[down.y][down.x] && is_valid(node, down) {
                    queue.push_back(down);
                    visited[down.y][down.x] = true;
                }
            }
        }

        Err(())
    }
}

pub fn part1(input: &str) -> usize {
    let terrain = Matrix::new(input);

    let s = terrain
        .rows
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            for (x, &square) in row.iter().enumerate() {
                if square == b'S' {
                    return Some((x, y));
                }
            }

            None
        })
        .unwrap();

    // game rules
    let height = |node: Node| match node.val {
        b'S' => b'a',
        b'E' => b'z',
        _ => node.val
    };

    let is_goal = |node: Node| node.val == b'E';

    let is_valid = |node: Node, next: Node| {
        let (h, n) = (height(node), height(next));

        h >= n || h.abs_diff(n) <= 1
    };

    terrain.bfs(s, is_goal, is_valid).unwrap()
}

pub fn part2(input: &str) -> usize {
    let terrain = Matrix::new(input);

    let s = terrain
        .rows
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            for (x, &square) in row.iter().enumerate() {
                if square == b'E' {
                    return Some((x, y));
                }
            }

            None
        })
        .unwrap();

    // game rules
    let height = |node: Node| match node.val {
        b'S' => b'a',
        b'E' => b'z',
        _ => node.val
    };

    let is_goal = |node: Node| height(node) == b'a';

    let is_valid = |node: Node, next: Node| {
        let (h, n) = (height(node), height(next));

        h <= n || h.abs_diff(n) <= 1
    };

    terrain.bfs(s, is_goal, is_valid).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "Sabqponm\n\
                              abcryxxl\n\
                              accszExk\n\
                              acctuvwj\n\
                              abdefghi\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 31);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 29);
    }
}

