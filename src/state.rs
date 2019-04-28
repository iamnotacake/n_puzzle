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
