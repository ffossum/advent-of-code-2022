use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::not_line_ending, combinator::*,
    sequence::preceded, Finish, IResult,
};

struct Directory {
    parent: Rc<RefCell<Directory>>,
    children: BTreeMap<String, Node>,
}

enum Node {
    File(u64),
    Directory(Directory),
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}
fn parse_command(i: &str) -> IResult<&str, Command> {
    fn parse_ls(i: &str) -> IResult<&str, Command> {
        map(tag("ls"), |_| Command::Ls)(i)
    }

    fn parse_cd(i: &str) -> IResult<&str, Command> {
        map(preceded(tag("cd "), not_line_ending), |d: &str| {
            Command::Cd(d.to_string())
        })(i)
    }
    let (i, _) = tag("$ ")(i)?;
    alt((parse_ls, parse_cd))(i)
}

#[derive(Debug)]
enum Entry {
    File(u64, String),
    Directory(String),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    fn parse_file(i: &str) -> IResult<&str, Entry> {
        let (i, size) = nom::character::complete::u64(i)?;
        let (i, _) = tag(" ")(i)?;
        let (i, name) = not_line_ending(i)?;
        Ok((i, Entry::File(size, name.to_string())))
    }
    fn parse_directory(i: &str) -> IResult<&str, Entry> {
        map(preceded(tag("dir "), not_line_ending), |name: &str| {
            Entry::Directory(name.to_string())
        })(i)
    }

    alt((parse_directory, parse_file))(i)
}

pub fn run() {
    let input = if cfg!(feature = "example") {
        include_str!("example.txt")
    } else {
        include_str!("input.txt")
    };

    let lines = input
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    for line in lines {
        println!("{line:?}");
    }
}
