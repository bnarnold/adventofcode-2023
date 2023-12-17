use core::fmt::Display;
use core::fmt::Formatter;
use core::str::FromStr;
use std::fmt::Write;

#[derive(Debug, Clone)]
enum DataState {
    Waiting(usize),
    Done,
}

#[derive(Debug, Clone, Default)]
struct KeyedPriorityQueue<Priority> {
    queue: Vec<(Priority, usize)>,
    data: Vec<DataState>,
}

impl<Priority: Ord> KeyedPriorityQueue<Priority> {
    fn swap_data_indices(&mut self, heap_index_1: usize, heap_index_2: usize) {
        let data_index_1 = self.queue[heap_index_1].1;
        let data_index_2 = self.queue[heap_index_2].1;
        let DataState::Waiting(stored_heap_index_1) = &mut self.data[data_index_1] else {
            unreachable!("Queue entry not waiting")
        };
        debug_assert_eq!(stored_heap_index_1, &heap_index_1);
        *stored_heap_index_1 = heap_index_2;
        let DataState::Waiting(stored_heap_index_2) = &mut self.data[data_index_2] else {
            unreachable!("Queue entry not waiting")
        };
        debug_assert_eq!(stored_heap_index_2, &heap_index_2);
        *stored_heap_index_2 = heap_index_1;
    }

    fn swap_remove(&mut self, heap_index: usize) -> Option<(Priority, usize)> {
        let heap_index_end = self.queue.len().checked_sub(1)?;
        self.swap_data_indices(heap_index, heap_index_end);
        let (priority, data_index) = self.queue.swap_remove(heap_index);
        let DataState::Waiting(stored_heap_index) = self.data.get(data_index)? else {
            unreachable!("Queue entry not waiting")
        };
        debug_assert_eq!(stored_heap_index, &heap_index_end);
        Some((priority, data_index))
    }

    fn swap(&mut self, heap_index_1: usize, heap_index_2: usize) {
        self.swap_data_indices(heap_index_1, heap_index_2);
        self.queue.swap(heap_index_1, heap_index_2);
    }

    fn heapify_up(&mut self, mut heap_index: usize) -> usize {
        while heap_index > 0 {
            let next_heap_index = (heap_index - 1) / 2;
            if self.queue[heap_index].0 < self.queue[next_heap_index].0 {
                self.swap(heap_index, next_heap_index);
                heap_index = next_heap_index;
            } else {
                return heap_index;
            }
        }
        heap_index
    }

    fn heapify_down(&mut self, mut heap_index: usize) -> usize {
        while 2 * heap_index + 1 < self.queue.len() {
            let left = 2 * heap_index + 1;
            let right = 2 * heap_index + 2;
            let next_heap_index =
                if right < self.queue.len() && self.queue[right].0 < self.queue[left].0 {
                    right
                } else {
                    left
                };
            if self.queue[next_heap_index].0 < self.queue[heap_index].0 {
                self.swap(heap_index, next_heap_index);
                heap_index = next_heap_index;
            } else {
                return heap_index;
            }
        }
        heap_index
    }

    pub fn update_priority(&mut self, data_index: usize, priority: Priority) {
        let entry = &mut self.data[data_index];
        match entry {
            DataState::Waiting(guard) if *guard == usize::MAX => {
                let heap_index = self.queue.len();
                *entry = DataState::Waiting(heap_index);
                self.queue.push((priority, data_index));
                self.heapify_up(heap_index);
            }
            DataState::Waiting(heap_index) => {
                let heap_index = *heap_index;
                let stored_priority = &mut self.queue[heap_index].0;
                if *stored_priority > priority {
                    *stored_priority = priority;
                    if self.heapify_up(heap_index) == heap_index {
                        self.heapify_down(heap_index);
                    }
                }
            }
            DataState::Done => {}
        }
    }

    pub fn pop(&mut self) -> Option<(Priority, usize)> {
        let (priority, data_index) = self.swap_remove(0)?;
        let entry = &mut self.data[data_index];
        match entry {
            DataState::Waiting(heap_index) => {
                debug_assert!(*heap_index < usize::MAX, "entry not in queue");
                debug_assert_eq!(heap_index, &self.queue.len());
                *entry = DataState::Done;
            }
            DataState::Done => unreachable!("Queue entry already done"),
        }
        self.heapify_down(0);
        Some((priority, data_index))
    }
}

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
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

#[derive(Debug)]
struct GraphNode(Pos, usize, Direction);

impl Grid<Field> {
    fn find_cart_path(&self, max_straight: u8, min_before_turn: u8) -> Option<u32> {
        let mut queue = KeyedPriorityQueue {
            queue: Vec::with_capacity(self.width),
            data: vec![DataState::Waiting(usize::MAX); self.data.len() * 2],
        };
        queue.update_priority(0, 0);
        queue.update_priority(1, 0);
        while let Some((cost, ix)) = queue.pop() {
            let pos = (ix / 2 % self.width, ix / 2 / self.width);
            let is_horizontal = ix % 2 == 1;
            if pos == (self.width - 1, self.height - 1) {
                return Some(cost.into());
            }
            for dir in if is_horizontal {
                [Direction::West, Direction::East]
            } else {
                [Direction::North, Direction::South]
            } {
                let mut next_pos = pos;
                let mut next_cost = cost;
                for i in 0..max_straight {
                    let Some(next) = self.step(next_pos, dir) else {
                        break;
                    };
                    next_pos = next;
                    next_cost += self.data[next.0 + self.width * next.1].cost();
                    if i + 1 >= min_before_turn {
                        queue.update_priority(
                            2 * (next_pos.0 + self.width * next_pos.1)
                                + if is_horizontal { 0 } else { 1 },
                            next_cost,
                        );
                    }
                }
            }
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
