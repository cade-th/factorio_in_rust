use crate::player::*;
use crate::render::*;
use crate::selector::*;
use crate::world::*;

#[derive(PartialEq)]
pub enum View {
    Editor,
    Minimap,
    FPS,
}

pub struct State {
    pub screen_width: i32,
    pub screen_height: i32,
    pub view: View,
    time_start: f64,
    delta_time: f32,
    last_tick_time: f64, // Tracks the last time we printed a message
}

impl State {
    pub fn new() -> Self {
        State {
            screen_width: 1024,
            screen_height: 1024,
            view: View::Minimap,
            time_start: unsafe { raylib::ffi::GetTime() },
            delta_time: 0.0,
            last_tick_time: unsafe { raylib::ffi::GetTime() }, // Initialize to current time
        }
    }

    // Fixed game time tick
    fn tick(&mut self) {
        let current_time = unsafe { raylib::ffi::GetTime() };

        // Check if one second has elapsed since the last tick
        if current_time - self.last_tick_time >= 1.0 {
            println!("One second has passed!");
            self.last_tick_time = current_time; // Reset the timer
        }
    }

    // Per frame update
    pub fn update(
        &mut self,
        renderer: &mut Renderer,
        player: &mut Player,
        selector: &mut Selector,
        world: &mut World,
    ) {
        self.delta_time = unsafe { raylib::ffi::GetFrameTime() };

        match self.view {
            View::Editor => {
                renderer.render_t = RendererType::Editor;
                selector.mov(self, world, &mut renderer.camera);
            }
            View::Minimap => {
                renderer.render_t = RendererType::Minimap;
                player.input_update(&mut renderer.camera, self, world);
            }
            View::FPS => {
                renderer.render_t = RendererType::FPS;
                player.input_update(&mut renderer.camera, self, world);
            }
        }
    }

    pub fn change_view(&mut self, view: View) {
        match view {
            View::Editor => self.view = View::Editor,
            View::Minimap => self.view = View::Minimap,
            View::FPS => self.view = View::FPS,
        }
    }
}
