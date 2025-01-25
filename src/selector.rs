use crate::state::*;
use crate::world::Blocks;
use crate::World;
use raylib::prelude::*;
use std::os::raw::c_int;

pub struct Selector {
    pub x: usize,
    pub y: usize,
}

impl Selector {
    pub fn new() -> Self {
        Selector { x: 0, y: 0 }
    }

    pub fn render(
        &self,
        d: &mut RaylibDrawHandle,
        texture_atlas: &Texture2D,
        world: &World,
        camera: &Camera2D,
    ) {
        let world_pos = Vector2::new(
            self.x as f32 * world.tile_size as f32,
            self.y as f32 * world.tile_size as f32,
        );

        let selector_screen_pos = Self::entity_to_screen(world_pos, camera);

        let selector_dest_rect = Rectangle {
            x: selector_screen_pos.x,
            y: selector_screen_pos.y,
            width: world.tile_size as f32 * camera.zoom,
            height: world.tile_size as f32 * camera.zoom,
        };

        let selector_texture_section = Rectangle {
            x: 0.0,
            y: 64.0,
            width: 32.0,
            height: 32.0,
        };

        d.draw_texture_pro(
            texture_atlas,
            selector_texture_section,
            selector_dest_rect,
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );
    }

    fn entity_to_screen(entity_pos: Vector2, camera: &Camera2D) -> Vector2 {
        Vector2::new(
            (entity_pos.x - camera.target.x) * camera.zoom + camera.offset.x,
            (entity_pos.y - camera.target.y) * camera.zoom + camera.offset.y,
        )
    }

    pub fn mov(&mut self, state: &mut State, world: &mut World, camera: &mut Camera2D) {
        unsafe {
            // Movement keys
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_K as c_int) {
                if self.y > 0 {
                    self.y = self.y.saturating_sub(1);
                }
            }
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_H as c_int) {
                if self.x > 0 {
                    self.x = self.x.saturating_sub(1);
                }
            }
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_J as c_int) {
                if self.y + 1 < world.data[0].len() {
                    self.y = self.y + 1;
                }
            }
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_L as c_int) {
                if self.x + 1 < world.data.len() {
                    self.x = self.x + 1;
                }
            }

            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_U as i32) {
                state.change_view(View::Minimap);
            }
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_I as i32) {
                state.change_view(View::Editor);
            }
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_O as i32) {
                state.change_view(View::FPS);
            }

            // Block placement/removal
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_W as c_int) {
                world.data[self.x][self.y] = Blocks::GRASS;
            }
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_R as i32) {
                world.data[self.x][self.y] = Blocks::STONE;
            }

            // Save world
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_S as c_int) {
                let _ = world.data_to_file("data.cade");
            }

            // Camera zoom adjustments
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_D as i32) {
                camera.zoom -= 0.05;
                println!("Zoom: {:.2}", camera.zoom);
            }
            if ffi::IsKeyPressed(ffi::KeyboardKey::KEY_F as i32) {
                camera.zoom += 0.05;
                println!("Zoom: {:.2}", camera.zoom);
            }

            camera.target.x = self.x as f32 * world.tile_size as f32;
            camera.target.y = self.y as f32 * world.tile_size as f32;
        }
    }
}
