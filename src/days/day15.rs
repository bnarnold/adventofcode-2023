use core::hash::Hash;
use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, Default)]
struct Bucket<T> {
    lenses: HashMap<T, (usize, u32)>,
    max_pos: usize,
}

impl<T: Hash + Eq> Bucket<T> {
    fn remove(&mut self, t: &T) {
        self.lenses.remove(t);
    }

    fn insert(&mut self, t: T, focal_length: u32) {
        match self.lenses.entry(t) {
            Entry::Occupied(mut e) => {
                e.get_mut().1 = focal_length;
            }
            Entry::Vacant(e) => {
                e.insert((self.max_pos, focal_length));
                self.max_pos += 1;
            }
        }
    }

    fn focal_length(&self) -> u32 {
        let mut values: Vec<_> = self.lenses.values().copied().collect();
        values.sort();
        values
            .into_iter()
            .enumerate()
            .map(|(i, (_, focal_length))| (i as u32 + 1) * focal_length)
            .sum()
    }
}

#[derive(Debug)]
struct BoxSet<'a>(Vec<Bucket<&'a str>>);

impl<'a> Default for BoxSet<'a> {
    fn default() -> Self {
        Self((0..256).map(|_| Bucket::default()).collect())
    }
}

impl<'a> BoxSet<'a> {
    fn process_step(&mut self, step: &'a str) {
        if let Some(label) = step.strip_suffix('-') {
            self.0[hash(label.bytes()) as usize].remove(&label);
        } else if let Some((label, focal_length)) = step.split_once('=') {
            self.0[hash(label.bytes()) as usize]
                .insert(label, focal_length.parse().expect("parse error"));
        }
    }

    fn focusing_power(&self) -> u32 {
        self.0
            .iter()
            .enumerate()
            .map(|(i, bucket)| (i as u32 + 1) * bucket.focal_length())
            .sum()
    }
}

fn hash(bytes: impl Iterator<Item = u8>) -> u8 {
    bytes.fold(0, |a: u8, b: u8| {
        (a.overflowing_add(b)).0.overflowing_mul(17).0
    })
}

fn level1(input: &str) -> u32 {
    input
        .split(',')
        .map(|chunk| hash(chunk.bytes()) as u32)
        .sum()
}

fn level2(input: &str) -> u32 {
    let mut box_set = BoxSet::default();
    for step in input.split(',') {
        box_set.process_step(step);
    }
    box_set.focusing_power()
}

fn main() {
    let test_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    println!("Level 1: {}", level1(test_input));
    println!("Level 2: {}", level2(test_input));
}
