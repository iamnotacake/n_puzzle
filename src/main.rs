extern crate crossterm_terminal;
extern crate n_puzzle;

use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::rc::Rc;

use crossterm_terminal::ClearType;
use n_puzzle::StrategyManhattan;
use n_puzzle::{MoveDirection, State};

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

fn solve_manhattan(state: State, goal: State) -> Option<Rc<StrategyManhattan>> {
    let goal_positions = goal.goal_positions();

    let mut heap = BinaryHeap::with_capacity(4096);
    let mut seen = HashSet::with_capacity(8192);

    let start = StrategyManhattan::new(state, MoveDirection::None, None, &goal_positions);
    heap.push(start);

    loop {
        if heap.len() % 10000 == 0 {
            eprintln!("Heap size {}", heap.len());
        }

        if let Some(curr) = heap.pop() {
            // eprintln!("heap -> score {}", curr.score);

            if curr.state == goal {
                eprintln!("Solved! Heap size {}, seen size {}", heap.len(), seen.len());
                return Some(curr);
            }

            for (direction, new_state) in curr.state.moves(curr.direction.clone()) {
                let next = StrategyManhattan::new(
                    new_state,
                    direction,
                    Some(curr.clone()),
                    &goal_positions,
                );

                if seen.contains(&next) {
                    continue;
                }

                // eprintln!("heap <- {}, heap size {}", next.score, heap.len());
                heap.push(next.clone());
                seen.insert(next);
            }
        } else {
            eprintln!("No more possible moves. Probably it's unsolvable");
            return None;
        }
    }
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

    solve_manhattan(state, goal);
}
