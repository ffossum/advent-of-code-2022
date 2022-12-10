use itertools::*;

#[derive(Debug, Clone)]
struct Crate {
    label: char,
}

#[derive(Debug, Clone)]
struct Stack {
    crates: Vec<Crate>,
}

impl Stack {
    fn new() -> Stack {
        Stack { crates: Vec::new() }
    }
    fn push(&mut self, crt: Crate) {
        self.crates.push(crt);
    }
    fn pop(&mut self) -> Option<Crate> {
        self.crates.pop()
    }

    fn top(&self) -> Option<&Crate> {
        self.crates.last()
    }
}

trait Crane {
    fn execute(&mut self, instruction: &Instruction);
}

#[derive(Debug)]
struct CrateMover9000 {
    stacks: Vec<Stack>,
}
impl Crane for CrateMover9000 {
    fn execute(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.move_count {
            self.move_single_crate(instruction.from_idx, instruction.to_idx);
        }
    }
}

impl CrateMover9000 {
    fn move_single_crate(&mut self, from_idx: usize, to_idx: usize) {
        let crt = self.stacks.get_mut(from_idx).unwrap().pop().unwrap();
        self.stacks.get_mut(to_idx).unwrap().push(crt);
    }
}

#[derive(Debug)]
struct Instruction {
    move_count: usize,
    from_idx: usize,
    to_idx: usize,
}
impl std::str::FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        words.next();
        let move_count: usize = words.next().unwrap().parse().unwrap();

        words.next();
        let from_idx: usize = words.next().unwrap().parse::<usize>().unwrap() - 1;

        words.next();
        let to_idx: usize = words.next().unwrap().parse::<usize>().unwrap() - 1;

        Ok(Instruction {
            move_count,
            from_idx,
            to_idx,
        })
    }
}

struct CrateMover9001 {
    stacks: Vec<Stack>,
}
impl Crane for CrateMover9001 {
    fn execute(&mut self, instruction: &Instruction) {
        let from_stack = self.stacks.get_mut(instruction.from_idx).unwrap();
        let mut moved_crates = from_stack.crates.split_off(from_stack.crates.len() - instruction.move_count);
        let to_stack = self.stacks.get_mut(instruction.to_idx).unwrap();
        to_stack.crates.append(&mut moved_crates);
    }
}

#[allow(dead_code)]
fn real_input() -> &'static str {
    include_str!("input.txt")
}

#[allow(dead_code)]
fn example_input() -> &'static str {
    r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#
}

pub fn run() {
    let lines = real_input().lines().collect::<Vec<_>>();
    let mut lines = lines.split(|l| l.is_empty());
    let mut crates = lines.next().unwrap().iter().rev();
    let instructions = lines.next().unwrap().iter();

    let size = (crates.next().unwrap().len() / 4) + 1;
    let mut stacks: Vec<Stack> = std::iter::repeat_with(|| Stack::new()).take(size).collect();

    for &line in crates {
        let crates = line.chars().chunks(4);
        for (idx, mut c) in Iterator::enumerate(crates.into_iter()) {
            c.next();
            if let Some(label) = c.next() {
                if label.is_alphabetic() {
                    stacks.get_mut(idx).unwrap().push(Crate { label });
                }
            }
        }
    }

    let mut crane = CrateMover9000 {
        stacks: stacks.clone(),
    };

    let instructions = instructions.map(|i| i.parse::<Instruction>().unwrap());

    for instruction in instructions.clone() {
        crane.execute(&instruction);
    }

    print!("Top crates: ");
    for stack in crane.stacks {
        let top_label = stack.top().map(|c| c.label).unwrap_or(' ');
        print!("{top_label}");
    }
    println!();

    let mut crane = CrateMover9001 { stacks };
    for instruction in instructions.clone() {
        crane.execute(&instruction);
    }
    print!("Top crates using CrateMover9001: ");
    for stack in crane.stacks {
        let top_label = stack.top().map(|c| c.label).unwrap_or(' ');
        print!("{top_label}");
    }
    println!();
}
