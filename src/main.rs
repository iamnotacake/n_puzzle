extern crate crossterm_terminal;
extern crate n_puzzle;

use std::io::prelude::*;

use crossterm_terminal::ClearType;
use n_puzzle::{MoveDirection, State};

fn solve(
    goal: &State,
    goal_positions: &Vec<(i32, i32)>,
    stack: &mut Vec<State>,
    level: usize,
    last_move: MoveDirection,
) -> bool {
    let state = &stack[stack.len() - 1];

    if level > 20 {
        return false;
    }

    if state == goal {
        return true;
    }

    let moves = state.moves(goal_positions);
    for (direction, dist, new_state) in moves {
        eprintln!(
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

        stack.push(new_state);

        if solve(goal, goal_positions, stack, level + 1, direction) {
            return true;
        } else {
            stack.pop();
        }
    }

    return false;
}

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
    let mut stack = Vec::with_capacity(128);
    stack.push(state.clone());

    let ret = solve(&goal, &goal_positions, &mut stack, 0, MoveDirection::None);

    if ret {
        println!("Solved with {} moves", stack.len() - 1);
    } else {
        println!("Not solved");
    }
}
