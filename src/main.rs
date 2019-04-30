extern crate argparse;
extern crate crossterm_terminal;
extern crate n_puzzle;

use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::rc::Rc;

use crossterm_terminal::ClearType;
use n_puzzle::StateDiff;
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

fn solve(
    state: State,
    goal: State,
    score_calculator: fn(&State, &Vec<(i32, i32)>) -> i32,
) -> Option<Rc<StateDiff>> {
    let goal_positions = goal.goal_positions();

    let mut heap = BinaryHeap::with_capacity(HEAP_SIZE_MAX + 1000);
    let mut seen = HashSet::with_capacity(HEAP_SIZE_MAX);

    let start = StateDiff::new(
        state,
        0,
        MoveDirection::None,
        None,
        score_calculator,
        &goal_positions,
    );
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
                eprintln!(
                    "Solved! heap size/cap {}/{}, seen size/cap {}/{}, moves count {}",
                    heap.len(),
                    heap.capacity(),
                    seen.len(),
                    seen.capacity(),
                    curr.level
                );
                return Some(curr);
            }

            for (direction, new_state) in curr.state.moves(curr.direction.clone()) {
                let next = StateDiff::new(
                    new_state,
                    curr.level + 1,
                    direction,
                    Some(curr.clone()),
                    score_calculator,
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

fn walk_states(sd: &Rc<StateDiff>) -> Vec<Rc<StateDiff>> {
    let mut vec = vec![sd.clone()];

    if let Some(prev) = &sd.prev_state {
        vec.extend(walk_states(prev));
    }

    vec
}

fn is_solvable(state: &State, goal: &State) -> bool {
    let mut inversions = 0;
    let s_table = state.table();
    let g_table = goal.table();

    for (idx, val) in s_table.iter().enumerate() {
        let pos_in_goal = g_table.iter().position(|x| x == val).unwrap();

        let have_on_right = &s_table[idx..];
        let need_on_left = &g_table[..pos_in_goal];

        let mut have_in_common = 0;
        for e in have_on_right {
            if need_on_left.iter().any(|x| x == e) {
                have_in_common += 1;
            }
        }

        inversions += have_in_common;
    }

    (inversions % 2) == 0
}

fn main() {
    let mut flag_interactive = false;
    let mut flag_interactive_delay = 100;

    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("N-puzzle solver");
        ap.refer(&mut flag_interactive).add_option(
            &["-i", "--interactive"],
            argparse::StoreTrue,
            "Show step-by-step solution. Default: false",
        );
        ap.refer(&mut flag_interactive_delay).add_option(
            &["-d", "--delay"],
            argparse::Store,
            "Delay between showing steps, ms. Default: 100",
        );
        ap.parse_args_or_exit();
    }

    let input = read_input();

    let mut iter = input
        .split_whitespace()
        .filter_map(|x| x.trim().parse::<i32>().ok());

    let size = iter.next().unwrap() as usize;
    let table: Vec<i32> = iter.take(size * size).collect();

    let terminal = crossterm_terminal::Terminal::new();

    let state = State::new(size, table);

    eprintln!("Have:");
    eprint!("{}", state);

    let goal = state.goal();
    eprintln!("Need:");
    eprint!("{}", goal);

    if !is_solvable(&state, &goal) {
        eprintln!("This thing is unsolvable!");
        return;
    }

    if let Some(last) = solve(state, goal, State::total_manhattan_dist) {
        if flag_interactive {
            std::thread::sleep_ms(2000);
            let states = walk_states(&last);

            for state in states.iter().rev() {
                terminal.clear(ClearType::All);
                print!("{}", state.state);
                std::thread::sleep_ms(flag_interactive_delay);
            }
        }
    }
}
