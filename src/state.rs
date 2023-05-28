use ggez::{
    event::EventHandler,
    graphics::{self, Color},
    GameError,
};

use crate::{FPS, GRID_SIZE};

pub struct State {
    grid: Vec<Vec<bool>>,
    fps: u32,
    running: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            grid: vec![vec![false; GRID_SIZE.1]; GRID_SIZE.0],
            fps: FPS,
            running: true,
        }
    }
}

impl EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(_ctx, Color::WHITE);
        canvas.finish(_ctx)?;
        Ok(())
    }
}
