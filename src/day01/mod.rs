fn get_elves() -> Vec<i32> {
    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<_>>();

    lines
        .split(|&l| l.is_empty())
        .map(|grp| {
            grp.iter()
                .map(|calories_str| calories_str.parse::<i32>().expect("invalid number"))
                .sum::<i32>()
        })
        .collect::<Vec<_>>()
}

pub fn part1() {
    let elves = get_elves();
    let max_calories = elves.iter().max().unwrap();
    println!("Max calories: {:?}", max_calories);
}
pub fn part2() {
    let mut elves = get_elves();
    elves.sort_unstable();
    let top3 = &elves[elves.len() - 3..];
    println!("Sum of top 3 calories: {:?}", top3.iter().sum::<i32>());
}
