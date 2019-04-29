extern crate crossterm_terminal;
extern crate n_puzzle;

use std::io::prelude::*;

use crossterm_terminal::ClearType;
use n_puzzle::{MoveDirection, State};

fn solve(
    goal: &State,
    goal_positions: &Vec<(i32, i32)>,
    stack: &mut Vec<(MoveDirection, State)>,
) -> bool {
    let (last_move, state) = &stack[stack.len() - 1];
    let last_move = last_move.clone();
    let level = stack.len() - 1;

    if level > 20 {
        return false;
    }

    if state == goal {
        return true;
    }

    let moves = state.moves(goal_positions);
    for (direction, dist, new_state) in moves {
        println!(
            "{}level {:4} total_distance {:2} direction {:?}",
            " ".repeat(level),
            level,
            dist,
            direction
        );

        match (last_move, direction) {
            (MoveDirection::Left, MoveDirection::Right)
            | (MoveDirection::Right, MoveDirection::Left)
            | (MoveDirection::Up, MoveDirection::Down)
            | (MoveDirection::Down, MoveDirection::Up) => {
                continue;
            }
            _ => {}
        }

        if stack.iter().any(|(_, prev_state)| prev_state == &new_state) {
            println!("{}SKIPPING PREV STATE", " ".repeat(level));
            continue;
        }

        stack.push((direction, new_state));

        if solve(goal, goal_positions, stack) {
            return true;
        } else {
            stack.pop();
        }
    }

    return false;
}

fn read_input() -> String {
    let mut input = String::with_capacity(2048);

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();

        if line.starts_with("#") {
            continue;
        }

        input.push_str(&line);
        input.push('\n');
    }

    input
}

fn main() {
    let input = read_input();

    let mut iter = input
        .split_whitespace()
        .filter_map(|x| x.trim().parse::<i32>().ok());

    let size = iter.next().unwrap() as usize;
    let table: Vec<i32> = iter.take(size * size).collect();

    let terminal = crossterm_terminal::Terminal::new();

    let state = State::new(size, table);

    // terminal.clear(ClearType::All);
    eprintln!("Have:");
    eprint!("{}", state);

    let goal = state.goal();
    eprintln!("Need:");
    eprint!("{}", goal);

    let goal_positions = goal.goal_positions();
    let mut stack = Vec::with_capacity(128);
    stack.push((MoveDirection::None, state.clone()));

    let ret = solve(&goal, &goal_positions, &mut stack);

    if ret {
        eprintln!("Solved with {} moves", stack.len() - 1);
    } else {
        eprintln!("Not solved");
    }
}
