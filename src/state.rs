use super::{manhattan_dist, MoveDirection};
use std::fmt;

#[derive(Clone)]
pub struct State {
    size: usize,
    table: Vec<i32>,
    // Position of empty (zero) cell
    empty: (i32, i32),
}

impl<'a> State {
    pub fn new(size: usize, table: Vec<i32>) -> State {
        assert_eq!(table.len(), size * size);

        let empty_index = table
            .iter()
            .enumerate()
            .filter(|(_idx, &val)| val == 0)
            .nth(0)
            .unwrap()
            .0;

        let empty = ((empty_index / size) as i32, (empty_index % size) as i32);

        State { size, table, empty }
    }

    fn cells(&'a self) -> StateCellIterator<'a> {
        StateCellIterator {
            state: &self,
            curr: 0,
            end: self.table.len() - 1,
        }
    }

    /// Return solved state so we can compare intermediate states with goal
    pub fn goal(&self) -> State {
        fn left_to_right(
            size: usize,
            table: &mut Vec<i32>,
            (y1, x1): (usize, usize),
            (y2, x2): (usize, usize),
            mut start: i32,
        ) {
            if y1 == y2 && x1 == x2 {
                return;
            }

            for x in x1..=x2 {
                table[y1 * size + x] = start;
                start += 1;
            }

            top_to_bottom(size, table, (y1 + 1, x1), (y2, x2), start);
        }

        fn top_to_bottom(
            size: usize,
            table: &mut Vec<i32>,
            (y1, x1): (usize, usize),
            (y2, x2): (usize, usize),
            mut start: i32,
        ) {
            if y1 == y2 && x1 == x2 {
                return;
            }

            for y in y1..=y2 {
                table[y * size + x2] = start;
                start += 1;
            }

            right_to_left(size, table, (y1, x1), (y2, x2 - 1), start);
        }

        fn right_to_left(
            size: usize,
            table: &mut Vec<i32>,
            (y1, x1): (usize, usize),
            (y2, x2): (usize, usize),
            mut start: i32,
        ) {
            if y1 == y2 && x1 == x2 {
                return;
            }

            for x in (x1..=x2).rev() {
                table[y2 * size + x] = start;
                start += 1;
            }

            bottom_to_top(size, table, (y1, x1), (y2 - 1, x2), start);
        }

        fn bottom_to_top(
            size: usize,
            table: &mut Vec<i32>,
            (y1, x1): (usize, usize),
            (y2, x2): (usize, usize),
            mut start: i32,
        ) {
            if y1 == y2 && x1 == x2 {
                return;
            }

            for y in (y1..=y2).rev() {
                table[y * size + x1] = start;
                start += 1;
            }

            left_to_right(size, table, (y1, x1 + 1), (y2, x2), start);
        }

        let mut table = vec![0; self.size * self.size];
        let end = self.size - 1;
        left_to_right(self.size, &mut table, (0, 0), (end, end), 1);

        State::new(self.size, table)
    }

    /// Returns vector where you put cell value as index and get target cell position.
    /// For 3x3 it would be:
    ///   [0] -> (1, 1)
    ///   [1] -> (0, 0)
    ///   [2] -> (0, 1)
    ///   [3] -> (0, 2)
    ///   and so on
    pub fn goal_positions(&self) -> Vec<(i32, i32)> {
        let mut res = vec![(0, 0); self.table.len()];

        for ((y, x), val) in self.cells() {
            res[val as usize] = (y, x);
        }

        res
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn at(&self, (y, x): (i32, i32)) -> i32 {
        self.table[y as usize * self.size + x as usize]
    }

    pub fn total_manhattan_dist(&self, goal: &Vec<(i32, i32)>) -> i32 {
        let mut total_dist = 0;

        for ((y, x), val) in self.cells() {
            // don't count bad empty cell position
            if val != 0 {
                total_dist += manhattan_dist((y, x), goal[val as usize]);
            }
        }

        total_dist
    }

    /// Returns list of possible moves, with distances calculated
    pub fn moves(&self, goal: &Vec<(i32, i32)>) -> Vec<(MoveDirection, i32, State)> {
        let mut res = Vec::with_capacity(4);
        let (y, x) = (self.empty.0 as usize, self.empty.1 as usize);

        if (self.empty.0 as usize) < self.size - 1 {
            // Up
            let mut table = self.table.clone();

            table[y * self.size + x] = table[(y + 1) * self.size + x];
            table[(y + 1) * self.size + x] = 0;

            let state = State {
                size: self.size,
                table,
                empty: (y as i32 + 1, x as i32),
            };

            res.push((MoveDirection::Up, state.total_manhattan_dist(goal), state));
        }

        if (self.empty.0 as usize) > 0 {
            // Down
            let mut table = self.table.clone();

            table[y * self.size + x] = table[(y - 1) * self.size + x];
            table[(y - 1) * self.size + x] = 0;

            let state = State {
                size: self.size,
                table,
                empty: (y as i32 - 1, x as i32),
            };

            res.push((MoveDirection::Down, state.total_manhattan_dist(goal), state));
        }

        if (self.empty.1 as usize) < self.size - 1 {
            // Left
            let mut table = self.table.clone();

            table[y * self.size + x] = table[y * self.size + x + 1];
            table[y * self.size + x + 1] = 0;

            let state = State {
                size: self.size,
                table,
                empty: (y as i32, x as i32 + 1),
            };

            res.push((MoveDirection::Left, state.total_manhattan_dist(goal), state));
        }

        if (self.empty.1 as usize) > 0 {
            // Right
            let mut table = self.table.clone();

            table[y * self.size + x] = table[y * self.size + x - 1];
            table[y * self.size + x - 1] = 0;

            let state = State {
                size: self.size,
                table,
                empty: (y as i32, x as i32 - 1),
            };

            res.push((
                MoveDirection::Right,
                state.total_manhattan_dist(goal),
                state,
            ));
        }

        res.sort_by_key(|(_direction, dist, _state)| dist.clone());

        res
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(
            f,
            "┌{}─────┐",
            "─────┬".repeat(self.size - 1)
        )?;

        for y in 0..self.size {
            for x in 0..self.size {
                let number = self.table[y * self.size + x];

                match number {
                    0 => write!(f, "│     ")?,
                    _ => write!(f, "│ {:3} ", number)?,
                }
            }
            writeln!(f, "│")?;

            if y < self.size - 1 {
                writeln!(
                    f,
                    "├{}─────┤",
                    "─────┼".repeat(self.size - 1)
                )?;
            }
        }

        writeln!(
            f,
            "└{}─────┘",
            "─────┴".repeat(self.size - 1)
        )?;

        Ok(())
    }
}

impl std::cmp::PartialEq<State> for State {
    fn eq(&self, other: &State) -> bool {
        self.table == other.table
    }
}

struct StateCellIterator<'a> {
    state: &'a State,
    curr: usize,
    end: usize,
}

impl<'a> Iterator for StateCellIterator<'a> {
    // (y, x), val
    type Item = ((i32, i32), i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr <= self.end {
            let y = self.curr / self.state.size;
            let x = self.curr % self.state.size;

            let old = self.curr;
            self.curr += 1;

            Some(((y as i32, x as i32), self.state.table[old]))
        } else {
            None
        }
    }
}
