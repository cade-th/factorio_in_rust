use crate::Player;
use crate::Selector;
use crate::State;
use crate::World;
use raylib::prelude::*;

// TODO:
// 1. Add a state machine for zooming to have discrete zoom levels to avoid screen tearing

pub enum RendererType {
    Editor,
    Minimap,
    FPS,
}

pub struct Renderer {
    pub render_t: RendererType,
    pub camera: Camera2D,
}

impl Renderer {
    pub fn new(renderer: RendererType) -> Self {
        Renderer {
            render_t: renderer,
            camera: Camera2D {
                target: Vector2::new(0.0, 0.0),
                offset: Vector2::new(0.0, 0.0),
                rotation: 0.0,
                zoom: 0.0,
            },
        }
    }

    pub fn render(
        &mut self,
        state: &State,
        d: &mut RaylibDrawHandle,
        texture_atlas: &Texture2D,
        world: &mut World,
        selector: &Selector,
        player: &Player,
    ) {
        match self.render_t {
            RendererType::Editor => {
                REditor::render(d, texture_atlas, world, &mut self.camera, &selector)
            }
            RendererType::Minimap => {
                RMinimap::render(state, d, texture_atlas, world, player, &mut self.camera)
            }
            RendererType::FPS => {
                RFPS::render(state, d, texture_atlas, world, player, &mut self.camera)
            }
        }
    }
}

pub struct RMinimap {}

impl RMinimap {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        state: &State,
        d: &mut RaylibDrawHandle,
        texture_atlas: &Texture2D,
        world: &mut World,
        player: &Player,
        camera: &mut Camera2D,
    ) {
        world.render(d, texture_atlas, camera);
        player.render(state, d, camera);
        camera.target = Vector2::new(player.pos.x as f32, player.pos.y as f32);
        camera.offset = Vector2::new(
            unsafe { raylib::ffi::GetScreenWidth() as f32 / 2.0 },
            unsafe { raylib::ffi::GetScreenHeight() as f32 / 2.0 },
        );
        camera.rotation = 0.0;
        camera.zoom = 1.0;
    }
}

pub struct REditor {}

impl REditor {
    fn render(
        d: &mut RaylibDrawHandle,
        texture_atlas: &Texture2D,
        world: &mut World,
        camera: &mut Camera2D,
        selector: &Selector,
    ) {
        d.draw_circle(200, 200, 20.0, Color::BLUE);
        world.render(d, texture_atlas, camera);
        selector.render(d, texture_atlas, world, camera);
    }
}

pub struct RFPS {
    pub camera: Camera2D,
}

impl RFPS {
    fn render(
        state: &State,
        d: &mut RaylibDrawHandle,
        texture_atlas: &Texture2D,
        world: &mut World,
        player: &Player,
        camera: &mut Camera2D,
    ) {
        player.render(state, d, camera);
        // d.draw_circle(200, 200, 20.0, Color::RED);
    }
}

pub fn entity_to_screen(entity_pos: Vector2, camera: &Camera2D) -> Vector2 {
    Vector2::new(
        (entity_pos.x - camera.target.x) * camera.zoom + camera.offset.x,
        (entity_pos.y - camera.target.y) * camera.zoom + camera.offset.y,
    )
}

// This works now
pub fn entity_to_world(entity: Vector2, world: &World) -> Vector2 {
    Vector2::new(
        (entity.x / world.tile_size as f32).floor(),
        (entity.y / world.tile_size as f32).floor(),
    )
}

pub fn world_to_entity(world_pos: Vector2, world: &World) -> Vector2 {
    Vector2::new(
        world_pos.x * world.tile_size as f32,
        world_pos.y * world.tile_size as f32,
    )
}
