use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

use gloo_console::log;

use crate::utils::Point;

pub struct Player {
    pub fire_cooldown: f64,
    pub has_fired: Option<Point>
}

const FIRE_COOLDOWN: f64 = 500.0;

impl Player {
    pub fn new() -> Self {
        Player {
            fire_cooldown: 0.0,
            has_fired: None
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

    pub fn fire(&mut self, x: f64, y: f64) {
        if self.fire_cooldown > 0.0 {
            return;
        }
        self.fire_cooldown = FIRE_COOLDOWN;
        self.has_fired = Some(Point{ x: x, y: y});
        // log!("Player fire triggered ", x, y);
    }

    pub fn get_fired(&mut self) -> Option<Point> {
        let ret = match &self.has_fired {
            Some(fp) => Some(Point{x: fp.x, y: fp.y}),
            None => None
        };
        self.has_fired = None;
        return ret;
    }

    pub fn render(&self, _ctx: CanvasRenderingContext2d) {
        // TODO: Draw the wand
    }
}