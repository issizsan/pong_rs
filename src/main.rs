use ggez;
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, Rect};
// use ggez::nalgebra as na;
use ggez::{Context, GameResult};

const SCREEN_WIDTH: f32 = 500.0;
const SCREEN_HEIGHT: f32 = 500.0;

const PADDLE_OFFSET: f32 = 10.0;
const PADDLE_WIDTH: f32 = 15.0;
const PADDLE_HEIGHT: f32 = 75.0;

const BALL_WIDTH: f32 = 23.5;

/* struct Paddle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
} */


#[derive(Debug)]
struct MainState {
    paddle1: Rect,
    paddle2: Rect,
    ball: Rect,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);

        let paddle1: Rect =
            graphics::Rect {
                x: (PADDLE_WIDTH / 2.0),
                y: screen_h_half,
                w: PADDLE_WIDTH,
                h: PADDLE_HEIGHT,
            };

        let paddle2: Rect =
            graphics::Rect {
                x: screen_w - PADDLE_WIDTH - PADDLE_OFFSET*0.75,
                y: screen_h_half,
                w: PADDLE_WIDTH,
                h: PADDLE_HEIGHT,
            };

        let ball: Rect =
            graphics::Rect {
                x: (screen_w_half - BALL_WIDTH),
                y: (screen_h_half + BALL_WIDTH),
                w: BALL_WIDTH,
                h: BALL_WIDTH,
            };


        let s = MainState {
            paddle1,
            paddle2,
            ball,
        };

        Ok(s)
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::W) {
            self.paddle1.y -= 5.0;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::S) {
            self.paddle1.y += 5.0;
        }

        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Up) {
            self.paddle2.y -= 5.0;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Down) {
            self.paddle2.y += 5.0;
        }

        return Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let paddle1_mesh = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            self.paddle1,
            graphics::WHITE
        )?;

        let paddle2_mesh = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            self.paddle2,
            graphics::WHITE
        )?;

        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            self.ball,
            graphics::WHITE
        )?;

        graphics::draw(ctx, &paddle1_mesh, graphics::DrawParam::default())?;
        graphics::draw(ctx, &paddle2_mesh, graphics::DrawParam::default())?;
        graphics::draw(ctx, &ball_mesh, graphics::DrawParam::default())?;

        graphics::present(ctx)?;

        Ok(())
    }

}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong", "ordaysan");
    let (ctx, event_loop) = &mut cb.build()?;

    graphics::set_drawable_size(ctx, SCREEN_WIDTH, SCREEN_HEIGHT)?;

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
