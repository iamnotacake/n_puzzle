extern crate crossterm_terminal;
extern crate n_puzzle;

use std::io::prelude::*;

use crossterm_terminal::ClearType;
use n_puzzle::State;

fn main() {
    let mut input = String::new();

    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input
        .split_whitespace()
        .filter_map(|x| x.trim().parse::<i32>().ok());

    let size = iter.next().unwrap() as usize;
    let table: Vec<i32> = iter.take(size * size).collect();

    let terminal = crossterm_terminal::Terminal::new();

    let state = State::new(size, table);

    // terminal.clear(ClearType::All);
    println!("Have:");
    print!("{}", state);

    let goal = state.goal();
    println!("Need:");
    print!("{}", goal);

    let goal_positions = goal.goal_positions();

    for (val, (y, x)) in goal_positions.iter().enumerate() {
        eprintln!("({}, {}) must be {}", y, x, val);
    }

    dbg!(state.total_manhattan_dist(&goal_positions));
}
