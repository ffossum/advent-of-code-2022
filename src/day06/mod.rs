use itertools::*;
use std::collections::*;

pub fn run() {
    let input = if cfg!(feature = "example") {
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
    } else {
        include_str!("input.txt")
    };

    let windows = input.chars().tuple_windows::<(_, _, _, _)>();
    let marker_idx = 4 + windows
        .map(|(a, b, c, d)| BTreeSet::from([a, b, c, d]))
        .take_while(|set| set.len() < 4)
        .count();

    println!("Start-of-packet marker after character {marker_idx}");

    let input = input.chars().collect_vec();
    let windows = input.windows(14);
    let message_idx = 14
        + windows
            .take_while(|chars| chars.iter().collect::<BTreeSet<_>>().len() < chars.len())
            .count();

    println!("Start-of-message marker after character {message_idx}");
}
