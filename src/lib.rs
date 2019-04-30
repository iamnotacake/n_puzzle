extern crate crossterm;

pub mod state;

pub use state::State;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MoveDirection {
    None,
    Up,
    Down,
    Left,
    Right,
}

#[inline]
fn linear_dist((y1, x1): (i32, i32), (y2, x2): (i32, i32)) -> f32 {
    let dy = (y1 - y2) as f32;
    let dx = (x1 - x2) as f32;

    f32::sqrt(dy * dy + dx * dx)
}

#[inline]
fn manhattan_dist((y1, x1): (i32, i32), (y2, x2): (i32, i32)) -> i32 {
    let dy = (y1 - y2).abs();
    let dx = (x1 - x2).abs();

    dy + dx
}

#[derive(Debug)]
pub struct StateDiff {
    pub score: i32,
    pub level: u16,
    pub state: State,
    pub direction: MoveDirection,
    pub prev_state: Option<(Rc<StateDiff>)>,
}

impl StateDiff {
    pub fn new(
        state: State,
        level: u16,
        direction: MoveDirection,
        prev_state: Option<(Rc<StateDiff>)>,
        score_calculator: fn(&State, &Vec<(i32, i32)>) -> i32,
        goal_positions: &Vec<(i32, i32)>,
    ) -> Rc<StateDiff> {
        Rc::new(StateDiff {
            score: -score_calculator(&state, &goal_positions),
            level,
            state,
            direction,
            prev_state,
        })
    }
}

impl PartialEq<StateDiff> for StateDiff {
    #[inline]
    fn eq(&self, other: &StateDiff) -> bool {
        self.score == other.score
    }
}

impl PartialOrd<StateDiff> for StateDiff {
    #[inline]
    fn partial_cmp(&self, other: &StateDiff) -> Option<Ordering> {
        if self.score < other.score {
            return Some(Ordering::Less);
        } else if self.score > other.score {
            return Some(Ordering::Greater);
        } else {
            return Some(Ordering::Equal);
        }
    }
}

impl Eq for StateDiff {}

impl Ord for StateDiff {
    #[inline]
    fn cmp(&self, other: &StateDiff) -> Ordering {
        if self.score < other.score {
            return Ordering::Less;
        } else if self.score > other.score {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
}

impl Hash for StateDiff {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state)
    }
}
