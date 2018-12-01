#![feature(dbg_macro)]

use std::collections::HashSet;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const INPUT: &'static str = include_str!("../1_input_1");

fn solve_1() -> impl Iterator<Item = isize> {
    INPUT
        .split("\n")
        .filter(|s| !s.trim().is_empty())
        .cycle()
        .scan(0, |c, s| {
            s.parse()
                .map(|x: isize| {
                    *c += x;
                    *c
                })
                .ok()
        })
}

fn solve_2() -> Result<isize> {
    let mut set = HashSet::new();
    set.insert(0);

    for state in solve_1() {
        if !set.insert(state) {
            return Ok(state);
        }
    }

    Err("No thing happens twice D:".into())
}

fn main() -> Result<()> {
    dbg!(solve_1()
        .nth(INPUT.split("\n").filter(|s| !s.trim().is_empty()).count() - 1)
        .ok_or("Couldn't parse input")?);
    dbg!(solve_2()?);

    Ok(())
}
