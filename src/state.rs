use std::fmt;

#[derive(Clone)]
pub struct State {
    size: usize,
    table: Vec<i32>,
    // Position of empty (zero) cell
    empty: (i32, i32),
}

impl State {
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

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn at(&self, (y, x): (i32, i32)) -> i32 {
        self.table[y as usize * self.size + x as usize]
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
