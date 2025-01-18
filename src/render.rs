use crate::Player;
use crate::World;
use raylib::prelude::*;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn render(
        &self,
        d: &mut RaylibDrawHandle,
        texture_atlas: &Texture2D,
        world: &World,
        player: &Player,
    ) {
        world.render(d, texture_atlas);
        player.render(d, texture_atlas);

        // Display camera information
        let player_text = format!("Player: ({:.2}, {:.2})", player.x, player.y);
        // Draw text on the screen
        d.draw_text(&player_text, 700, 10, 20, Color::WHITE);
    }
}
