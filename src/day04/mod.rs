use core::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug)]
struct Sections(RangeInclusive<i32>);

impl Sections {
    fn fully_contains(&self, other: &Sections) -> bool {
        self.0.start() <= other.0.start() && self.0.end() >= other.0.end()
    }

    fn overlaps(&self, other: &Sections) -> bool {
        other.0.contains(&self.0.start())
            || other.0.contains(&self.0.end())
            || self.0.contains(&other.0.start())
            || self.0.contains(&other.0.end())
    }
}

impl FromStr for Sections {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once("-").ok_or("must specify range with '-'")?;
        let from: i32 = from.parse().map_err(|_| "invalid range start")?;
        let to: i32 = to.parse().map_err(|_| "invalid range end")?;

        Ok(Sections(from..=to))
    }
}
fn real_input() -> &'static str {
    include_str!("input.txt")
}

#[allow(dead_code)]
fn example_input() -> &'static str {
    r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#
}

pub fn part1() {
    let input = real_input().lines();

    let sections = input.map(|s| {
        let (a, b) = s.split_once(",").unwrap();
        let a: Sections = a.parse().unwrap();
        let b: Sections = b.parse().unwrap();
        (a, b)
    });

    let fully_contained = sections
        .filter(|(a, b)| a.fully_contains(b) || b.fully_contains(a))
        .collect::<Vec<_>>();

    println!(
        "Number of pairs where one range contains the other: {}",
        fully_contained.len()
    );
}

pub fn part2() {
    let input = real_input().lines();

    let sections = input.map(|s| {
        let (a, b) = s.split_once(",").unwrap();
        let a: Sections = a.parse().unwrap();
        let b: Sections = b.parse().unwrap();
        (a, b)
    });

    let overlapping = sections.filter(|(a, b)| a.overlaps(b)).collect::<Vec<_>>();

    println!(
        "Number of pairs where ranges overlap: {}",
        overlapping.len()
    );
}
