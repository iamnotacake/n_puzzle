use std::fmt;
use std::io::prelude::*;

#[derive(Clone)]
struct State {
    size: usize,
    table: Vec<i32>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "┌─")?;
        for _ in 0..self.size {
            write!(f, "───")?;
        }
        writeln!(f, "┐")?;

        for y in 0..self.size {
            write!(f, "│ ")?;
            for x in 0..self.size {
                let number = self.table[y * self.size + x];
                match number {
                    0 => write!(f, "   ")?,
                    _ => write!(f, "{:2} ", number)?,
                }
            }
            writeln!(f, "│")?;
        }

        write!(f, "└─")?;
        for _ in 0..self.size {
            write!(f, "───")?;
        }
        writeln!(f, "┘")?;

        Ok(())
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
    assert_eq!(table.len(), size * size);

    let state = State { size, table };
    print!("{}", state);
}
