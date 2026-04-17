use web_sys::WebGlRenderingContext as GL;
use web_sys::window;

use crate::model::camera::Camera;

pub struct ViewManager {
    pub camera: Camera,
    first_update: bool,
    pub delta: f64,
    last_update: f64,
    pub u_time: f32,
    pub width: i32,
    pub height: i32,
}

impl ViewManager {
    pub fn new() -> Self {
        let mut camera = Camera::new();
        camera.move_camera(0.0, 1.0,-9.0);
        camera.look_at(0.0, 0.0, 0.0);
        camera.set_up(0.0, 1.0, 0.0);

        ViewManager {
            camera: camera,
            first_update: true,
            delta: 0.0,
            last_update: 0.0,
            u_time: 0.0,
            width: 0,
            height: 0,
        }
    }

    pub fn update(&mut self, _gl: &GL) {
        let now = instant::now();
        if self.first_update {
            self.first_update = false;
            self.last_update = now;
            return;
        }
        if self.last_update > now {
            gloo_console::log!("Time went backwards, skipping frame");
            return;
        }
        let diff = now - self.last_update;
        self.delta = diff / 1000.0;
        self.u_time += self.delta as f32;

        self.width = window().unwrap().inner_width().unwrap().as_f64().unwrap() as i32;
        self.height = window().unwrap().inner_height().unwrap().as_f64().unwrap() as i32;
        
        self.camera.update_screen_dimensions(self.width as f32, self.height as f32);
        self.camera.refresh(self.delta);
        self.last_update = now;


    }
}