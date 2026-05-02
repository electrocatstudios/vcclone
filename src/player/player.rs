use web_sys::WebGlRenderingContext as GL;

use crate::{assets::firebolt::Firebolt, utils::Location3D};
use super::keymanager::KeyManager;

pub struct Player {
    location: Location3D,
    look_rot: (f32, f32)
}

// const FIRE_COOLDOWN: f64 = 500.0;
const PLAYER_SPEED: f32 = 0.005;
const PLAYER_MAX_X: f32 = 1.9;
const PLAYER_MIN_X: f32 = -1.9;

impl Player {
    pub fn new() -> Self {
        Player {
           location: Location3D::new(0.0, 1.0, -9.0),
           look_rot: (0.0, 0.0),
        }
    }

    pub fn update(&mut self, delta: f64, key_manager: &KeyManager) {
        if key_manager.is_key_down("a") {
            self.location.x += PLAYER_SPEED * delta as f32;
        }
        if key_manager.is_key_down("d") {
            self.location.x -= PLAYER_SPEED * delta as f32;
        }

        // Clamp the player's x position
        if self.location.x > PLAYER_MAX_X {
            self.location.x = PLAYER_MAX_X;
        }
        if self.location.x < PLAYER_MIN_X {
            self.location.x = PLAYER_MIN_X;
        }
    }

    pub fn get_location(&self) -> &Location3D {
        &self.location
    }

    pub fn get_look_rotation(&self) -> (f32, f32) {
        self.look_rot
    }

    pub fn render(&self, _ctx: GL) {
        // TODO: Draw the wand
    }

    pub fn look(&mut self, x_diff: f32, y_diff: f32) {
        self.look_rot.0 += x_diff * 0.005;
        self.look_rot.1 += y_diff * 0.005;

        // Clamp the vertical look rotation to prevent flipping
        if self.look_rot.0 > std::f32::consts::FRAC_PI_2 {
            self.look_rot.0 = std::f32::consts::FRAC_PI_2;
        }
        if self.look_rot.0 < -std::f32::consts::FRAC_PI_2 {
            self.look_rot.0 = -std::f32::consts::FRAC_PI_2;
        }

        if self.look_rot.1 > std::f32::consts::PI {
            self.look_rot.1 = std::f32::consts::PI;
        }
        if self.look_rot.1 < -std::f32::consts::PI * 1.2 {
            self.look_rot.1 = -std::f32::consts::PI * 1.2;
        }
        // gloo_console::log!("Look rotation: ", self.look_rot.0, self.look_rot.1);
    }

    pub fn cast_firebolt(&self) -> Firebolt {
        let speed = 0.01;
        // let velocity = (
        //     self.look_rot.1.sin() * self.look_rot.0.cos() * speed,
        //     self.look_rot.0.sin() * speed,
        //     self.look_rot.1.cos() * self.look_rot.0.cos() * speed,
        // );
        let velocity = (
            0.0,
            0.0,
            speed
        );
        Firebolt::new(self.location.clone(), velocity)
    }
}