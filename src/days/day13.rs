use std::iter::zip;

#[derive(Debug)]
struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Eq> Grid<T> {
    fn horizontal_smudges(&self, y: usize) -> usize {
        ((2 * y).saturating_sub(self.height)..y)
                .map(|y1| {
                    let y2 = 2 * y - 1 - y1;
                    zip(
                        &self.data[(y1 * self.width)..((y1 + 1) * self.width)],
                        &self.data[(y2 * self.width)..((y2 + 1) * self.width)],
                    )
                    .filter(|(a, b)| a != b)
                    .count()
                })
                .sum()
    }

    fn vertical_smudges(&self, x: usize) -> usize {
        self
                .data
                .chunks_exact(self.width)
                .map(|row| {
                    zip(
                        &row[(2 * x).saturating_sub(self.width)..x],
                        row[x..(2 * x).min(self.width)].iter().rev(),
                    )
                    .filter(|(a, b)| a != b)
                    .count()
                })
                .sum()
    }
}

fn parse_pattern(pattern: &str) -> Grid<char> {
    let mut height = 0;
    let data: Vec<_> = pattern
        .lines()
        .filter(|line| !line.trim().is_empty())
        .flat_map(|line| {
            height += 1;
            line.trim().chars()
        })
        .collect();
    let width = data.len() / height;
    Grid {
        data,
        width,
        height,
    }
}

fn reflection_count(input: &str, smudges: usize) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            let grid = parse_pattern(pattern);
            let horizontal = (1..grid.height).find(|y| grid.horizontal_smudges(*y) == smudges);
            let vertical = (1..grid.width).find(|x| grid.vertical_smudges(*x) == smudges);
            horizontal.unwrap_or_default() * 100 + vertical.unwrap_or_default()
        })
        .sum()
}

fn main() {
    let input = r"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
    println!("Level 1: {}", reflection_count(input, 0));
    println!("Level 2: {}", reflection_count(input, 1));
}
