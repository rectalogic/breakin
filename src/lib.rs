use avian3d::prelude::*;

pub mod app;
mod arcball;
mod ball;
mod bricks;
mod player;

#[derive(PhysicsLayer, Default)]
enum GameLayer {
    #[default]
    Default,
    Brick,
    Ball,
    Paddle,
}
