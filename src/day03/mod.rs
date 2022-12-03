use std::collections::HashSet;

fn read_input() -> &'static str {
    include_str!("input.txt")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Item(char);
impl Item {
    fn priority(&self) -> i32 {
        match self.0 {
            c @ 'a'..='z' => c as i32 - 'a' as i32 + 1,
            c @ 'A'..='Z' => c as i32 - 'A' as i32 + 27,
            _ => panic!("invalid item: {:?}", self),
        }
    }
}

#[cfg(test)]
mod item_tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(Item('a').priority(), 1);
        assert_eq!(Item('p').priority(), 16);
        assert_eq!(Item('L').priority(), 38);
        assert_eq!(Item('P').priority(), 42);
        assert_eq!(Item('v').priority(), 22);
        assert_eq!(Item('t').priority(), 20);
        assert_eq!(Item('s').priority(), 19);
    }
}

#[derive(Debug)]
struct Rucksack {
    compartments: [Compartment; 2],
}
impl Rucksack {
    fn shared_item(&self) -> &Item {
        let [a, b] = &self.compartments;
        let a = a.item_set();
        let b = b.item_set();

        let mut shared = a.intersection(&b);
        *shared.next().unwrap()
    }

    fn item_set(&self) -> HashSet<&Item> {
        self.compartments
            .iter()
            .fold(HashSet::new(), |result, compartment| {
                let other: HashSet<&Item> = compartment.item_set();
                result.union(&other).copied().collect()
            })
    }
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        let (a, b) = s.split_at(s.len() / 2);
        Self {
            compartments: [Compartment::from(a), Compartment::from(b)],
        }
    }
}

#[derive(Debug)]
struct Compartment(Vec<Item>);
impl From<&str> for Compartment {
    fn from(s: &str) -> Self {
        Self(s.chars().map(Item).collect())
    }
}

impl Compartment {
    fn item_set(&self) -> HashSet<&Item> {
        self.0.iter().collect()
    }
}

#[allow(dead_code)]
fn example_input() -> &'static str {
    r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#
}

pub fn part1() {
    let input = read_input().lines();
    // let input = example_input().lines();

    let rucksacks = input.map(Rucksack::from).collect::<Vec<_>>();
    let shared_items = rucksacks
        .iter()
        .map(|r| r.shared_item())
        .collect::<Vec<_>>();

    let sum: i32 = shared_items.iter().map(|i| i.priority()).sum();
    println!("Sum of item priorities: {}", sum)
}

pub fn part2() {
    use itertools::Itertools;

    let input = read_input().lines();
    // let input = example_input().lines();

    let groups: Vec<Group> = input
        .map(Rucksack::from)
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|grp| Group(grp.into_iter().collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    let badges = groups.into_iter().map(|g| g.badge()).collect::<Vec<_>>();
    let sum: i32 = badges.iter().map(|b| b.priority()).sum();

    println!("Sum of badges' priorities: {}", sum);
}

#[derive(Debug)]
struct Group(Vec<Rucksack>);

impl Group {
    fn badge(&self) -> Item {
        let shared_items: HashSet<&Item> = self
            .0
            .iter()
            .map(|r| r.item_set())
            .reduce(|a, b| a.intersection(&b).copied().collect())
            .unwrap();

        shared_items.into_iter().copied().next().unwrap()
    }
}
