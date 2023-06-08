mod state;
use crate::state::State;
use ggez::{conf::WindowMode, event, ContextBuilder, GameResult};

const CELL_SIZE: (f32, f32) = (20.0, 20.0);
const GRID_SIZE: (usize, usize) = (40, 40);
const WINDOW_SIZE: (f32, f32) = (
    CELL_SIZE.0 * GRID_SIZE.0 as f32,
    CELL_SIZE.1 * GRID_SIZE.1 as f32,
);
const FPS: u32 = 10;

fn main() -> GameResult {
    let state = State::new();

    let cb = ContextBuilder::new("GOL", "yetyetanotherusername")
        .window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1));
    let (context, event_loop) = cb.build().unwrap();
    event::run(context, event_loop, state);
}
