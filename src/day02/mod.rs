#[derive(PartialEq, Eq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}
impl Hand {
    fn beats(&self, other: &Hand) -> bool {
        match (self, other) {
            (Hand::Rock, Hand::Scissors)
            | (Hand::Paper, Hand::Rock)
            | (Hand::Scissors, Hand::Paper) => true,
            _ => false,
        }
    }
    fn score(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn from(opponent_hand: &Hand, outcome: &Outcome) -> Hand {
        match (outcome, opponent_hand) {
            (Outcome::Draw, _) => *opponent_hand,

            (Outcome::Win, Hand::Rock) => Hand::Paper,
            (Outcome::Win, Hand::Paper) => Hand::Scissors,
            (Outcome::Win, Hand::Scissors) => Hand::Rock,

            (Outcome::Loss, Hand::Rock) => Hand::Scissors,
            (Outcome::Loss, Hand::Paper) => Hand::Rock,
            (Outcome::Loss, Hand::Scissors) => Hand::Paper,
        }
    }
}

impl TryFrom<char> for Hand {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Hand::Rock),
            'B' | 'Y' => Ok(Hand::Paper),
            'C' | 'Z' => Ok(Hand::Scissors),

            c => Err(format!("Invalid hand: {:?}", c)),
        }
    }
}

struct Strategy {
    opponent_hand: Hand,
    my_hand: Hand,
}
impl Strategy {
    fn from(opponent_hand: Hand, outcome: &Outcome) -> Self {
        Strategy {
            opponent_hand,
            my_hand: Hand::from(&opponent_hand, outcome),
        }
    }

    fn score(&self) -> i32 {
        self.my_hand.score()
            + if self.my_hand.beats(&self.opponent_hand) {
                6
            } else if self.my_hand == self.opponent_hand {
                3
            } else {
                0
            }
    }
}

fn read_input() -> impl Iterator<Item = (char, char)> {
    include_str!("input.txt").lines().map(|line| {
        let mut chars = line.chars();
        let a: char = chars.next().unwrap();
        chars.next();
        let b: char = chars.next().unwrap();
        (a, b)
    })
}

pub fn part1() {
    let strategies = read_input().map(|(a, b)| {
        let opponent_hand: Hand = a.try_into().unwrap();
        let my_hand: Hand = b.try_into().unwrap();
        Strategy {
            opponent_hand,
            my_hand,
        }
    });

    let score: i32 = strategies.map(|strat| strat.score()).sum();

    println!("Total score according to strategy guide: {}", score);
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl TryFrom<char> for Outcome {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),

            c => Err(format!("Invalid outcome: {:?}", c)),
        }
    }
}
pub fn part2() {
    let strategies = read_input().map(|(a, b)| {
        let opponent_hand: Hand = a.try_into().unwrap();
        let outcome: Outcome = b.try_into().unwrap();
        Strategy::from(opponent_hand, &outcome)
    });

    let score: i32 = strategies.map(|strat| strat.score()).sum();

    println!("Total score according to revised strategy guide: {}", score);
}
