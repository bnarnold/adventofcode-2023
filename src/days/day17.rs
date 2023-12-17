use core::cmp::Ordering;
use core::fmt::Display;
use core::fmt::Formatter;
use core::num::NonZeroU32;
use core::str::FromStr;
use std::collections::BinaryHeap;
use std::fmt::Write;

#[derive(Debug, Clone, Copy)]
struct Field(u8);

impl Field {
    fn cost(self) -> u32 {
        self.0 as u32
    }
}

impl From<&Field> for char {
    fn from(field: &Field) -> char {
        (b'0' + field.0).into()
    }
}

impl TryFrom<char> for Field {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let digit = c.to_digit(10).ok_or(())? as u8;
        if digit == 0 {
            Err(())
        } else {
            Ok(Field(digit))
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn index(self) -> usize {
        match self {
            Direction::North => 0,
            Direction::West => 1,
            Direction::South => 2,
            Direction::East => 3,
        }
    }

    fn rotate_left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn rotate_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }

    fn next_directions(self, remaining_steps: u8, max_straight: u8, min_before_turn: u8) -> impl Iterator<Item = (Self, u8)> {
        [
            remaining_steps.checked_sub(1).map(|steps| (self, steps)),
            (remaining_steps + min_before_turn <= max_straight).then_some((self.rotate_left(), max_straight - 1)),
            (remaining_steps + min_before_turn <= max_straight).then_some((self.rotate_right(), max_straight - 1)),
        ]
        .into_iter()
        .flatten()
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

#[derive(Debug, Clone)]
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

#[derive(Debug, PartialEq, Eq)]
struct Step(Pos, u8, Direction, u32);

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        self.3.cmp(&other.3).reverse()
    }
}

impl Grid<Field> {
    fn find_cart_path(&self, max_straight: u8, min_before_turn: u8) -> Option<u32> {
        let mut costs: Vec<Option<NonZeroU32>> = vec![None; self.data.len() * max_straight as usize * 4];
        let mut queue = BinaryHeap::new();
        queue.push(Step((1, 0), max_straight - 1, Direction::East, self.data[1].cost()));
        queue.push(Step(
            (0, 1),
            max_straight - 1,
            Direction::South,
            self.data[self.width].cost()
        ));
        while let Some(Step(pos, remaining_steps, dir, cost)) = queue.pop() {
            if pos == (self.width - 1, self.height - 1) && remaining_steps + min_before_turn <= max_straight {
                return Some(cost.into());
            }
            let stored_cost = &mut costs
                [4 * max_straight as usize * (pos.0 + pos.1 * self.width) + 4 * remaining_steps as usize + dir.index()];
            if let Some(stored_cost) = stored_cost {
                if u32::from(*stored_cost) <= cost {
                    continue;
                }
            }
            *stored_cost = cost.try_into().ok();
            queue.extend(
                dir.next_directions(remaining_steps, max_straight, min_before_turn)
                    .filter_map(|(dir, remaining_steps)| {
                        Some((self.step(pos, dir)?, dir, remaining_steps))
                    })
                    .map(|(pos, dir, remaining_steps)| {
                        let new_cost = 
                            cost + self.data[pos.0 + self.width * pos.1].cost();
                        Step(
                            pos,
                            remaining_steps,
                            dir,
                            new_cost,
                        )
                    }),
            )
        }
        return None;
    }

    fn step(&self, pos: Pos, dir: Direction) -> Option<Pos> {
        let dir: (isize, isize) = dir.into();
        let next_pos = (
            pos.0.checked_add_signed(dir.0)?,
            pos.1.checked_add_signed(dir.1)?,
        );
        (next_pos.0 < self.width && next_pos.1 < self.height).then_some(next_pos)
    }
}

fn level1(input: &str) -> u32 {
    let grid: Grid<Field> = input.parse().expect("Parse error");
    grid.find_cart_path(3, 1).expect("No path found")
}

fn level2(input: &str) -> u32 {
    let grid: Grid<Field> = input.parse().expect("Parse error");
    grid.find_cart_path(10, 4).expect("No path found")
}

fn main() {
    let input = r"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
    let now = std::time::Instant::now();
    println!("{}", level1(input));
    println!("{}", level2(input));
    println!("{:?}", now.elapsed());
}
