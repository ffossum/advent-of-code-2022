mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    println!("Day 1:");
    day01::part1();
    day01::part2();

    println!("\nDay 2:");
    day02::part1();
    day02::part2();

    println!("\nDay 3:");
    day03::part1();
    day03::part2();

    println!("\nDay 4:");
    day04::part1();
    day04::part2();

    println!("\nDay 5:");
    day05::run();

    println!("\nDay 6:");
    day06::run();

    println!("\nDay 7:");
    day07::run();
}
