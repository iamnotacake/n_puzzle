pub mod state;

pub use state::State;

#[derive(Copy, Clone, Debug)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

// #[inline]
// fn linear_dist<F>((y1, x1): (F, F), (y2, x2): (F, F)) -> f32
// where
//     F: std::convert::Into<f32>,
// {
//     let dy = y1.into() - y2.into();
//     let dx = x1.into() - x2.into();
//
//     f32::sqrt(dy * dy + dx * dx)
// }

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
