extern crate crossterm_terminal;
extern crate n_puzzle;

use std::io::prelude::*;

use crossterm_terminal::ClearType;
use n_puzzle::{MoveDirection, State};

fn solve(
    state: &State,
    goal: &State,
    goal_positions: &Vec<(i32, i32)>,
    level: usize,
    last_move: Option<MoveDirection>,
) {
    if level > 20 {
        return;
    }

    if state == goal {
        panic!("SOLVED!");
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

        solve(
            &new_state,
            goal,
            goal_positions,
            level + 1,
            Some(*direction),
        );
    }
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

    solve(&state, &goal, &goal_positions, 0, None);
}
