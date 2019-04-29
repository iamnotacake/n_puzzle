extern crate crossterm_terminal;
extern crate n_puzzle;

use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::rc::Rc;

use crossterm_terminal::ClearType;
use n_puzzle::StrategyManhattan;
use n_puzzle::{MoveDirection, State};

const HEAP_SIZE_MAX: usize = 500_000;
const HEAP_SIZE_SHRINK: usize = 50_000;

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

fn shrink_heap<T>(mut heap: BinaryHeap<T>) -> BinaryHeap<T>
where
    T: Ord,
{
    let mut new_heap = BinaryHeap::with_capacity(HEAP_SIZE_MAX);

    for _ in 0..HEAP_SIZE_SHRINK {
        new_heap.push(heap.pop().unwrap());
    }

    new_heap
}

fn solve_manhattan(state: State, goal: State) -> Option<Rc<StrategyManhattan>> {
    let goal_positions = goal.goal_positions();

    let mut heap = BinaryHeap::with_capacity(HEAP_SIZE_MAX + 1000);
    let mut seen = HashSet::with_capacity(HEAP_SIZE_MAX);

    let start = StrategyManhattan::new(state, MoveDirection::None, None, &goal_positions);
    heap.push(start);

    loop {
        if heap.len() >= HEAP_SIZE_MAX {
            eprintln!(
                "heap size {}, shrinking to {}, seen size {}",
                heap.len(),
                HEAP_SIZE_SHRINK,
                seen.len()
            );
            heap = shrink_heap(heap);
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
