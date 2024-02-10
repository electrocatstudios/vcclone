use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

use gloo_console::log;
pub struct Player {
    pub fire_cooldown: f64,
}

const FIRE_COOLDOWN: f64 = 500.0;

impl Player {
    pub fn new() -> Self {
        Player {
            fire_cooldown: 0.0,
        }
    }

    pub fn update(&mut self, delta: f64) {
        if self.fire_cooldown > 0.0 {
            self.fire_cooldown -= delta;
            
        }
        if self.fire_cooldown < 0.0 {            
            self.fire_cooldown = 0.0;
        }
    }

    pub fn fire(&mut self) {
        self.fire_cooldown = FIRE_COOLDOWN;
    }

    pub fn render(&self, _ctx: CanvasRenderingContext2d) {
        // TODO: Draw the wand
    }
}