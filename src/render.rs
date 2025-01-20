use crate::Player;
use crate::World;
use raylib::prelude::*;

// TODO:
// 1. Add a state machine for zooming to have discrete zoom levels to avoid screen tearing

pub struct Renderer {
    pub camera: Camera2D,
}

impl Renderer {
    pub fn new(player: &Player) -> Self {
        let offset = unsafe {
            Vector2::new(
                raylib::ffi::GetScreenWidth() as f32 / 2.0,
                raylib::ffi::GetScreenHeight() as f32 / 2.0,
            )
        };

        Renderer {
            camera: Camera2D {
                target: Vector2::new(player.x as f32, player.y as f32),
                offset,
                rotation: 0.0,
                zoom: 1.0,
            },
        }
    }

    pub fn render(
        &self,
        d: &mut RaylibDrawHandle,
        texture_atlas: &Texture2D,
        world: &World,
        player: &Player,
    ) {
        world.render(d, texture_atlas, &self.camera);
        player.render(d, texture_atlas, &self.camera, world);
    }
}
