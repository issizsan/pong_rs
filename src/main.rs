use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

const SCREEN_WIDTH: f32 = 500.0;
const SCREEN_HEIGHT: f32 = 500.0;

const PADDLE_OFFSET: f32 = 20.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 70.0;

/* struct Paddle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
} */

struct MainState { 
    paddle1: Mesh,
    paddle2: Mesh,
}

impl MainState { 
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);


        let paddle1: Mesh = Mesh::new_rectangle (
            ctx,
            DrawMode::fill(),
            graphics::Rect {
                x: (PADDLE_WIDTH / 2.0) + PADDLE_OFFSET as f32,
                y: screen_h_half,
                w: PADDLE_WIDTH,
                h: PADDLE_HEIGHT,
            },
            graphics::WHITE,
        )?;

        let paddle2: Mesh = Mesh::new_rectangle (
            ctx,
            DrawMode::fill(),
            graphics::Rect {
                x: (SCREEN_WIDTH as f32 - PADDLE_OFFSET as f32),
                y: screen_h_half,
                w: PADDLE_WIDTH,
                h: PADDLE_HEIGHT,
            },
            graphics::WHITE,
        )?;

        let s = MainState {
            paddle1,
            paddle2,
        };

        Ok(s)
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        graphics::draw(ctx, &self.paddle1, graphics::DrawParam::default())?;
        graphics::draw(ctx, &self.paddle2, graphics::DrawParam::default())?;

        graphics::present(ctx)?;

        Ok(())
    }

}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong", "ordaysan");
    let (ctx, event_loop) = &mut cb.build()?;

    graphics::set_drawable_size(ctx, SCREEN_WIDTH, SCREEN_HEIGHT);

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}