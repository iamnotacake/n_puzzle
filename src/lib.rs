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

/// Strategy based on sum of manhattan distances.
/// The less is sum, the more score this state gets.
#[derive(Debug)]
pub struct StrategyManhattan {
    pub score: i32,
    pub level: u16,
    pub state: State,
    pub direction: MoveDirection,
    pub prev_state: Option<(Rc<StrategyManhattan>)>,
}

impl StrategyManhattan {
    pub fn new(
        state: State,
        level: u16,
        direction: MoveDirection,
        prev_state: Option<(Rc<StrategyManhattan>)>,
        score_calculator: fn(&State, &Vec<(i32, i32)>) -> i32,
        goal_positions: &Vec<(i32, i32)>,
    ) -> Rc<StrategyManhattan> {
        Rc::new(StrategyManhattan {
            score: -score_calculator(&state, &goal_positions),
            level,
            state,
            direction,
            prev_state,
        })
    }
}

impl PartialEq<StrategyManhattan> for StrategyManhattan {
    #[inline]
    fn eq(&self, other: &StrategyManhattan) -> bool {
        self.score == other.score
    }
}

impl PartialOrd<StrategyManhattan> for StrategyManhattan {
    #[inline]
    fn partial_cmp(&self, other: &StrategyManhattan) -> Option<Ordering> {
        if self.score < other.score {
            return Some(Ordering::Less);
        } else if self.score > other.score {
            return Some(Ordering::Greater);
        } else {
            return Some(Ordering::Equal);
        }
    }
}

impl Eq for StrategyManhattan {}

impl Ord for StrategyManhattan {
    #[inline]
    fn cmp(&self, other: &StrategyManhattan) -> Ordering {
        if self.score < other.score {
            return Ordering::Less;
        } else if self.score > other.score {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
}

impl Hash for StrategyManhattan {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state)
    }
}
