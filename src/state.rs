use ggez::{
    event::EventHandler,
    glam::vec2,
    graphics::{self, Color, DrawMode, Mesh, Rect},
    mint::Point2,
    GameError,
};

use crate::{CELL_SIZE, FPS, GRID_SIZE};

pub struct State {
    pub grid: Vec<Vec<bool>>,
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
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        while ctx.time.check_update_time(self.fps) {
            let mut coords: Vec<(usize, usize)> = vec![];

            // for i in 0..GRID_SIZE.0 {
            //     let left = if i > 0 { i - 1 } else { GRID_SIZE.0 - 1 };
            //     let right = if i < GRID_SIZE.0 - 1 { i + 1 } else { 0 };
            //
            //     for j in 0..GRID_SIZE.1 {
            //         let up = if j > 0 { j - 1 } else { GRID_SIZE.1 - 1 };
            //         let down = if j < GRID_SIZE.1 - 1 { j + 1 } else { 0 };
            //
            //
            //     }
            //
            //     }
            // }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        for i in 0..GRID_SIZE.0 {
            for j in 0..GRID_SIZE.1 {
                if self.grid[i][j] {
                    let cv = vec2(i as f32 * CELL_SIZE.0, j as f32 * CELL_SIZE.1);
                    let rect = Mesh::new_rectangle(
                        ctx,
                        DrawMode::fill(),
                        Rect::new(0.0, 0.0, CELL_SIZE.0, CELL_SIZE.1),
                        Color::BLACK,
                    )?;
                    canvas.draw(
                        &rect,
                        graphics::DrawParam::new().dest(cv).color(Color::BLACK),
                    );
                }
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}
