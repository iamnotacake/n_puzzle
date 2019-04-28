extern crate crossterm_terminal;
extern crate n_puzzle;

use std::io::prelude::*;

use crossterm_terminal::ClearType;
use n_puzzle::{MoveDirection, State};

fn solve(
    goal: &State,
    goal_positions: &Vec<(i32, i32)>,
    stack: &mut Vec<(MoveDirection, i32, State)>,
    state: &State,
    level: usize,
    last_move: Option<MoveDirection>,
) -> bool {
    if level > 20 {
        return false;
    }

    if state == goal {
        return true;
    }

    let moves = state.moves(goal_positions);
    for (direction, dist, new_state) in &moves {
        println!(
            "level {:4} direction {:?}  \ttotal_distance {}",
            level, direction, dist
        );

        match (last_move, direction) {
            (Some(MoveDirection::Left), MoveDirection::Right)
            | (Some(MoveDirection::Right), MoveDirection::Left)
            | (Some(MoveDirection::Up), MoveDirection::Down)
            | (Some(MoveDirection::Down), MoveDirection::Up) => {
                continue;
            }
            _ => {}
        }

        // stack.push((*direction, *dist, *new_state));

        if solve(
            goal,
            goal_positions,
            stack,
            &new_state,
            level + 1,
            Some(*direction),
        ) {
            return true;
        } else {
            // stack.pop();
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

    let ret = solve(&goal, &goal_positions, &mut stack, &state, 0, None);

    if ret {
        println!("Solved with {} moves", stack.len());
    } else {
        println!("Not solved");
    }
}
