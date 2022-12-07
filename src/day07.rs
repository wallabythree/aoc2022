// Idea for tree arena:
// https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6
// https://fkohlgrueber.github.io/blog/tree-structure-of-file-systems/
//
// However, please don't blame them for this mess.

enum NodeType {
    Directory { children: Vec<usize> },
    File { size: usize }
}

struct TreeNode {
    index: usize,
    name: String,
    parent: Option<usize>,
    value: NodeType
}

struct TreeArena {
    arena: Vec<TreeNode>
}

impl TreeArena {
    fn push(&mut self, mut node: TreeNode) -> Result<usize, ()> {
        let index = self.arena.len();
        node.index = index;

        self.arena.push(node);

        if let Some(parent) = self.arena[index].parent {
            if let NodeType::Directory { children } = &mut self.arena[parent].value {
                children.push(index);
            }
        }

        Ok(index)
    }

    // recursive node size calculation function
    fn size(&self, index: usize) -> usize {
        match &self.arena[index].value {
            NodeType::Directory { children }  => {
                children
                    .iter()
                    .map(|child_index| self.size(*child_index))
                    .sum()
            },
            NodeType::File { size } => {
                *size
            }
        }      
    }
}



fn parse_filetree(input: &str) -> TreeArena {
    // initialize tree
    let mut tree = TreeArena { arena: Vec::new() };

    // add root directory and set working directory
    let mut cwd = tree.push(
        TreeNode { 
            index: 0,
            name: String::from("/"),
            parent: None,
            value: NodeType::Directory { children: vec![] }
        }).unwrap();

    input
        .trim()
        .split('\n')
        .for_each(|line| {
            let args = line.split(' ').collect::<Vec<_>>();

            if args[0].starts_with('$') {
                match args[1] {
                    "cd" => match args[2] {
                        "/" => cwd = 0,
                        ".." => if let Some(index) = tree.arena[cwd].parent {
                            cwd = index;
                        },
                        _ => {
                            let new_directory = TreeNode {
                                index: 0,
                                name: args[2].to_string(),
                                parent: Some(cwd),
                                value: NodeType::Directory { children: vec![] }
                            };
                            if let Ok(index) = tree.push(new_directory) {
                                cwd = index;
                            }

                        }
                    },
                    _ => ()
                }
            } else if args[0].chars().nth(0).unwrap().is_digit(10) {
                let (name, size) = (
                    args[1].to_string(),
                    args[0].parse::<usize>().unwrap()
                );

                let file = TreeNode {
                    index: 0,
                    name,
                    parent: Some(cwd),
                    value: NodeType::File { size }
                };

                tree.push(file).unwrap();
            }
        });

    tree
}

pub fn part1(input: &str) -> usize {
    let tree = parse_filetree(input);

    tree
        .arena
        .iter()
        .filter(|entry| match entry.value {
            NodeType::Directory{ children: _ } => true,
            _ => false
        })
        .map(|directory| tree.size(directory.index))
        .filter(|size| *size <= 100000)
        .sum()
}

pub fn part2(input: &str) -> usize {
    const TOTAL: usize = 70000000;
    const NEEDED: usize = 30000000;

    let tree = parse_filetree(input);

    let used = tree.size(0);

    let mut dir_sizes = tree
        .arena
        .iter()
        .filter(|entry| match entry.value {
            NodeType::Directory{ children: _ } => true,
            _ => false
        })
        .map(|directory| tree.size(directory.index))
        .collect::<Vec<_>>();

    dir_sizes.sort();

    *dir_sizes.iter().find(|size| TOTAL - used + *size >= NEEDED).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day07::{part1, part2};

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 95437);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 24933642)
    }
}

