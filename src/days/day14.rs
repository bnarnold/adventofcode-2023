use core::fmt::Display;
use core::fmt::Formatter;
use std::collections::{hash_map::Entry, HashMap};
use std::fmt::Write;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Copy, Clone)]
enum Field {
    Round,
    Cube,
    Empty,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl Display for Grid<Field> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for row in self.data.chunks_exact(self.width) {
            for piece in row {
                f.write_char(match piece {
                    Field::Round => 'O',
                    Field::Cube => '#',
                    Field::Empty => ' ',
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn parse_grid(input: &str) -> Grid<Field> {
    let mut height = 0;
    let data: Vec<_> = input
        .lines()
        .flat_map(|line| {
            height += 1;
            line.trim().chars().map(|c| match c {
                'O' => Field::Round,
                '#' => Field::Cube,
                '.' => Field::Empty,
                _ => unimplemented!(),
            })
        })
        .collect();
    let width = data.len() / height;
    Grid {
        data,
        width,
        height,
    }
}

impl Grid<Field> {
    fn total_load(&self) -> usize {
        self.data
            .chunks_exact(self.width)
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().filter_map(move |piece| {
                    matches!(piece, Field::Round).then_some(self.height - i)
                })
            })
            .sum()
    }

    fn rotate_west(&mut self) {
        for row in self.data.chunks_exact_mut(self.width) {
            for chunk in row.split_mut(|p| matches!(p, Field::Cube)) {
                if chunk.is_empty() {
                    continue;
                }
                let mut left = 0;
                let mut right = chunk.len() - 1;
                while left < right {
                    if matches!(&chunk[left], Field::Empty) {
                        if matches!(&chunk[right], Field::Round) {
                            chunk.swap(left, right);
                        }
                        right -= 1;
                    } else {
                        left += 1;
                    }
                }
            }
        }
    }

    fn rotate_east(&mut self) {
        for row in self.data.chunks_exact_mut(self.width) {
            for chunk in row.split_mut(|p| matches!(p, Field::Cube)) {
                if chunk.is_empty() {
                    continue;
                }
                let mut left = 0;
                let mut right = chunk.len() - 1;
                while left < right {
                    if matches!(&chunk[left], Field::Round) {
                        if matches!(&chunk[right], Field::Empty) {
                            chunk.swap(left, right);
                        }
                        right -= 1;
                    } else {
                        left += 1;
                    }
                }
            }
        }
    }

    fn rotate_north(&mut self) {
        for i in 0..self.width {
            let mut chunk_start = 0;
            while chunk_start < self.height {
                let chunk_end = (chunk_start..self.height)
                    .find(|j| matches!(self.data[i + j * self.width], Field::Cube))
                    .unwrap_or(self.height);

                let mut bottom = chunk_start;
                let mut top = chunk_end.saturating_sub(1);
                while bottom < top {
                    let bottom_pos = i + bottom * self.width;
                    let top_pos = i + top * self.width;
                    if matches!(self.data[bottom_pos], Field::Empty) {
                        if matches!(self.data[top_pos], Field::Round) {
                            self.data.swap(bottom_pos, top_pos);
                        }
                        top -= 1;
                    } else {
                        bottom += 1;
                    }
                }
                chunk_start = chunk_end + 1;
            }
        }
    }

    fn rotate_south(&mut self) {
        for i in 0..self.width {
            let mut chunk_start = 0;
            while chunk_start < self.height {
                let chunk_end = (chunk_start..self.height)
                    .find(|j| matches!(self.data[i + j * self.width], Field::Cube))
                    .unwrap_or(self.height);

                let mut bottom = chunk_start;
                let mut top = chunk_end.saturating_sub(1);
                while bottom < top {
                    let bottom_pos = i + bottom * self.width;
                    let top_pos = i + top * self.width;
                    if matches!(self.data[bottom_pos], Field::Round) {
                        if matches!(self.data[top_pos], Field::Empty) {
                            self.data.swap(bottom_pos, top_pos);
                        }
                        top -= 1;
                    } else {
                        bottom += 1;
                    }
                }
                chunk_start = chunk_end + 1;
            }
        }
    }

    fn cycle(&mut self) {
        self.rotate_north();
        self.rotate_west();
        self.rotate_south();
        self.rotate_east();
    }
}

fn main() {
    let input = r"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"
    .trim();
    let mut grid = parse_grid(input);
    grid.rotate_north();
    println!("Level 1: {}", grid.total_load());
    let mut cache: HashMap<_, usize> = HashMap::new();
    for i in 0..1_000_000_000 {
        match cache.entry(grid.data.clone()) {
            Entry::Occupied(e) => {
                let j = e.get();
                for _ in 0..((1_000_000_000 - j) % (i - j)) {
                    grid.cycle();
                }
                break;
            }
            Entry::Vacant(e) => {
                e.insert(i);
            }
        }
        grid.cycle();
    }

    println!("Level 2: {}", grid.total_load());
}
