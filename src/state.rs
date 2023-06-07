use ggez::{
    event::EventHandler,
    graphics::{self, Canvas, Color, DrawMode, DrawParam, Mesh, Rect},
    Context, GameError, GameResult,
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

    fn draw_state(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        for i in 0..GRID_SIZE.0 {
            for j in 0..GRID_SIZE.1 {
                if self.grid[i][j] {
                    let rect = self.new_mesh(ctx, i, j).unwrap();
                    canvas.draw(&rect, DrawParam::default());
                }
            }
        }
    }

    fn new_mesh(
        &mut self,
        ctx: &mut Context,
        x_coord: usize,
        y_coord: usize,
    ) -> Result<Mesh, GameError> {
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(
                x_coord as f32 * CELL_SIZE.0,
                y_coord as f32 * CELL_SIZE.1,
                CELL_SIZE.0,
                CELL_SIZE.1,
            ),
            Color::BLACK,
        )
    }

    fn update_grid(&mut self) {
        let mut coords: Vec<(usize, usize)> = vec![];

        for i in 0..GRID_SIZE.0 {
            let left = if i > 0 { i - 1 } else { GRID_SIZE.0 - 1 };
            let right = if i < GRID_SIZE.0 - 1 { i + 1 } else { 0 };

            for j in 0..GRID_SIZE.1 {
                let up = if j > 0 { j - 1 } else { GRID_SIZE.1 - 1 };
                let down = if j < GRID_SIZE.1 - 1 { j + 1 } else { 0 };

                let mut sum = 0;

                sum += self.grid[left][up] as u8;
                sum += self.grid[i][up] as u8;
                sum += self.grid[right][up] as u8;
                sum += self.grid[right][j] as u8;
                sum += self.grid[right][down] as u8;
                sum += self.grid[i][down] as u8;
                sum += self.grid[left][down] as u8;
                sum += self.grid[left][j] as u8;

                if self.grid[i][j] && (sum < 2 || sum > 3) {
                    coords.push((i, j))
                } else if !self.grid[i][j] && sum == 3 {
                    coords.push((i, j));
                };
            }
        }

        for coord in coords {
            self.grid[coord.0][coord.1] ^= true;
        }
    }
}

impl EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(self.fps) {
            self.update_grid();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        self.draw_state(ctx, &mut canvas);
        canvas.finish(ctx)?;
        Ok(())
    }
}
