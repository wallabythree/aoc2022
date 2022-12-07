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

impl TreeNode {
    fn new_file(name: String, size: usize, parent: Option<usize>) -> Self {
        Self {
            index: 0,
            name,
            parent,
            value: NodeType::File { size }
        }
    }

    fn new_directory(name: String, parent: Option<usize>) -> Self {
        Self {
            index: 0,
            name,
            parent,
            value: NodeType::Directory { children: vec![] }
        }
    }
}

struct TreeArena {
    arena: Vec<TreeNode>
}

impl TreeArena {
    fn push(&mut self, mut node: TreeNode) -> usize {
        let index = self.arena.len();
        node.index = index;

        self.arena.push(node);

        if let Some(parent) = self.arena[index].parent {
            if let NodeType::Directory { 
                children
            } = &mut self.arena[parent].value {
                children.push(index);
            }
        }

        index
    }

    // calculate node size recursively
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
    let root_dir = TreeNode::new_directory("/".to_string(), None);
    let mut cwd = tree.push(root_dir);

    // parse input line by line
    input
        .trim()
        .split('\n')
        .for_each(|line| {
            let args = line.split(' ').collect::<Vec<_>>();

            if args[0] == "$"  && args[1] == "cd" { 
                // navigate directories
                match args[2] {
                    "/" => cwd = 0,
                    ".." => if let Some(index) = tree.arena[cwd].parent {
                        cwd = index;
                    },
                    _ => {
                        // add new directory to tree
                        let name = args[2].to_string();
                        let new_dir = TreeNode::new_directory(name, Some(cwd));

                        cwd = tree.push(new_dir);
                    }
                }

            } else if args[0].as_bytes()[0].is_ascii_digit() {
                // add new file to tree
                let (name, size) = (
                    args[1].to_string(),
                    args[0].parse::<usize>().unwrap()
                );

                let file = TreeNode::new_file(name, size, Some(cwd));

                tree.push(file);
            }
        });

    tree
}

pub fn part1(input: &str) -> usize {
    let tree = parse_filetree(input);

    tree
        .arena
        .iter()
        .filter(|entry| matches!(entry.value, NodeType::Directory {children: _}))
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
        .filter(|entry| matches!(entry.value, NodeType::Directory {children: _}))
        .map(|directory| tree.size(directory.index))
        .collect::<Vec<_>>();

    dir_sizes.sort();

    *dir_sizes
        .iter()
        .find(|size| TOTAL - used + *size >= NEEDED)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day07::{part1, part2};

    const TEST_INPUT: &str = "$ cd /\n\
                              $ ls\n\
                              dir a\n\
                              14848514 b.txt\n\
                              8504156 c.dat\n\
                              dir d\n\
                              $ cd a\n\
                              $ ls\n\
                              dir e\n\
                              29116 f\n\
                              2557 g\n\
                              62596 h.lst\n\
                              $ cd e\n\
                              $ ls\n\
                              584 i\n\
                              $ cd ..\n\
                              $ cd ..\n\
                              $ cd d\n\
                              $ ls\n\
                              4060174 j\n\
                              8033020 d.log\n\
                              5626152 d.ext\n\
                              7214296 k\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 95437);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 24933642)
    }
}

