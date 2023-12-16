use core::fmt::Display;
use core::fmt::Formatter;
use core::str::FromStr;
use std::collections::VecDeque;
use std::fmt::Write;
#[derive(Debug)]
enum Piece {
    Empty,
    MirrorAigu,
    MirrorGrave,
    VSplitter,
    HSplitter,
}

#[derive(Debug)]
struct Field(Piece, u8);

impl From<&Field> for char {
    fn from(field: &Field) -> char {
        match field {
            Field(Piece::Empty, energized) => match (energized & 0b101, energized & 0b1010) {
                (0, 0) => ' ',
                (0, _) => '═',
                (_, 0) => '║',
                (_, _) => '╬',
            },
            Field(Piece::MirrorAigu, _) => '/',
            Field(Piece::MirrorGrave, _) => '\\',
            Field(Piece::VSplitter, _) => '┃',
            Field(Piece::HSplitter, _) => '━',
        }
    }
}

impl TryFrom<char> for Field {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(Field(
            match c {
                '.' => Piece::Empty,
                '\\' => Piece::MirrorGrave,
                '/' => Piece::MirrorAigu,
                '-' => Piece::HSplitter,
                '|' => Piece::VSplitter,
                _ => return Err(()),
            },
            0,
        ))
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn index(&self) -> u8 {
        match self {
            Direction::North => 0,
            Direction::West => 1,
            Direction::South => 2,
            Direction::East => 3,
        }
    }
}

impl From<Direction> for (isize, isize) {
    fn from(dir: Direction) -> (isize, isize) {
        match dir {
            Direction::North => (0, -1),
            Direction::West => (-1, 0),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
        }
    }
}

impl Field {
    fn next_directions(&mut self, dir: Direction) -> impl Iterator<Item = Direction> {
        let mut next_dirs = [None, None];
        let mask = 1 << dir.index();
        if self.1 & mask == 0 {
            self.1 |= mask;
            match self.0 {
                Piece::Empty => {
                    next_dirs[0] = Some(dir);
                }
                Piece::MirrorAigu => {
                    next_dirs[0] = Some(match dir {
                        Direction::North => Direction::East,
                        Direction::West => Direction::South,
                        Direction::South => Direction::West,
                        Direction::East => Direction::North,
                    });
                }
                Piece::MirrorGrave => {
                    next_dirs[0] = Some(match dir {
                        Direction::North => Direction::West,
                        Direction::West => Direction::North,
                        Direction::South => Direction::East,
                        Direction::East => Direction::South,
                    });
                }
                Piece::HSplitter => match dir {
                    Direction::North | Direction::South => {
                        next_dirs = [Some(Direction::West), Some(Direction::East)]
                    }
                    _ => next_dirs[0] = Some(dir),
                },
                Piece::VSplitter => match dir {
                    Direction::East | Direction::West => {
                        next_dirs = [Some(Direction::North), Some(Direction::South)]
                    }
                    _ => next_dirs[0] = Some(dir),
                },
            }
        }
        next_dirs.into_iter().flatten()
    }
}

#[derive(Debug)]
struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Display for Grid<T>
where
    for<'a> &'a T: Into<char>,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for row in self.data.chunks_exact(self.width) {
            for field in row {
                f.write_char(field.into())?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl<T: TryFrom<char>> FromStr for Grid<T> {
    type Err = T::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::new();
        let mut height = 0;
        for line in input.trim().lines() {
            for c in line.trim().chars() {
                data.push(c.try_into()?)
            }
            height += 1;
        }
        let width = data.len() / height;
        Ok(Self {
            data,
            width,
            height,
        })
    }
}

type Pos = (usize, usize);

impl Grid<Field> {
    fn energize(&mut self, start_pos: usize, start_dir: Direction) {
        let mut queue = VecDeque::new();
        let start_pos = match start_dir {
            Direction::North => (start_pos, self.height - 1),
            Direction::West => (self.width - 1, start_pos),
            Direction::South => (start_pos, 0),
            Direction::East => (0, start_pos),
        };
        queue.push_back((start_pos, start_dir));
        while let Some((pos, dir)) = queue.pop_front() {
            queue.extend(self.energize_field(pos, dir));
        }
    }

    fn step(&self, pos: Pos, dir: Direction) -> Option<Pos> {
        let dir: (isize, isize) = dir.into();
        let next_pos = (
            pos.0.checked_add_signed(dir.0)?,
            pos.1.checked_add_signed(dir.1)?,
        );
        (next_pos.0 < self.width && next_pos.1 < self.height).then_some(next_pos)
    }

    // I know this is wrong, but too lazy to capture correctly
    fn energize_field(
        &mut self,
        pos: Pos,
        dir: Direction,
    ) -> impl Iterator<Item = (Pos, Direction)> + '_ {
        self.data[pos.0 + self.width * pos.1]
            .next_directions(dir)
            .filter_map(move |next_dir| Some((self.step(pos, next_dir)?, next_dir)))
    }

    fn reset(&mut self) {
        for field in &mut self.data {
            field.1 = 0;
        }
    }

    fn energized_count(&self) -> usize {
        self.data
            .iter()
            .filter(|Field(_, energized)| *energized != 0)
            .count()
    }
}

fn level1(input: &str) -> usize {
    let mut grid: Grid<Field> = input.parse().expect("Parse error");
    grid.energize(0, Direction::East);
    println!("{grid}");
    grid.energized_count()
}

fn level2(input: &str) -> usize {
    let mut grid: Grid<Field> = input.parse().expect("Parse error");
    (0..grid.width)
        .flat_map(|i| [(i, Direction::South), (i, Direction::North)])
        .chain((0..grid.height).flat_map(|j| [(j, Direction::East), (j, Direction::West)]))
        .map(|(pos, dir)| {
            grid.reset();
            grid.energize(pos, dir);
            grid.energized_count()
        })
        .max()
        .unwrap()
}

fn main() {
    let input = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";
    let grid: Grid<Field> = input.parse().unwrap();
    println!("{grid}");
    println!("{}", level1(input));
    println!("{}", level2(input));
}
