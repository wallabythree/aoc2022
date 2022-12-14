struct Grid<'a> {
    rows: Vec<&'a [u8]>,
    cols: Vec<Vec<u8>>,
}

impl<'a> Grid<'a> {

    fn new(input: &'a str) -> Grid<'a> {
        let rows = input
            .trim()
            .as_bytes()
            .split(|&b| b == b'\n')
            .collect::<Vec<_>>();

        let cols = (0..rows.len())
            .map(|col| rows.iter().map(move |row| row[col]).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { rows, cols }
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let tree = self.rows[y][x];

        x == 0 || x == self.rows.len() - 1
        || y == 0 || y == self.cols.len() - 1
        || self.rows[y][..x].iter().all(|&neighbour| tree > neighbour) 
        || self.rows[y][(x + 1)..].iter().all(|&neighbour| tree > neighbour)
        || self.cols[x][..y].iter().all(|&neighbour| tree > neighbour)
        || self.cols[x][(y + 1)..].iter().all(|&neighbour| tree > neighbour)
    }

    fn score(&self, x: usize, y: usize) -> usize {
        let tree = &self.rows[y][x];

        let mut left = self.rows[y][..x].iter().rev();
        let mut right = self.rows[y][(x + 1)..].iter();
        let mut above = self.cols[x][..y].iter().rev();
        let mut below = self.cols[x][(y + 1)..].iter();

        let lscore = left
            .position(|neighbour| neighbour >= tree)
            .map_or(x, |position| position + 1);

        let rscore = right
            .position(|neighbour| neighbour >= tree)
            .map_or(self.rows[y].len() - 1 - x, |position| position + 1);
        
        let ascore = above
            .position(|neighbour| neighbour >= tree)
            .map_or(y, |position| position + 1);

        let bscore = below
            .position(|neighbour| neighbour >= tree)
            .map_or(self.cols[x].len() - 1 - y, |position| position + 1);

        lscore * rscore * ascore * bscore
    }
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::new(input);

    let mut visible = 0;

    for y in 0..grid.rows.len() {
        for x in 0..grid.rows.len() {
            if grid.is_visible(x, y) {
                visible += 1;
            }
        }
    }

    visible
}

pub fn part2(input: &str) -> usize {
    let grid = Grid::new(input);

    let mut max = 0;

    for y in 0..grid.rows.len() {
        for x in 0..grid.rows[y].len() {
            let score = grid.score(x, y);

            if score > max {
                max = score;
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use crate::day08::{part1, part2};

    const TEST_INPUT: &str = "30373\n\
                              25512\n\
                              65332\n\
                              33549\n\
                              35390\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 21);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 8)
    }
}

