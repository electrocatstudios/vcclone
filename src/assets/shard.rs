use web_sys::WebGlRenderingContext as GL;

use crate::{model::model::Model, utils::Location3D};
use crate::model::camera::Camera;
use crate::consts::*;

pub struct Shard {
    pub position: Location3D,
    pub velocity: Location3D,
    pub model: Model,
    pub color: [f32; 3],
}

impl Shard {
    pub fn new(name: String) -> Self {
        let mut model = Model::new(name);
        model.set_gltf(include_str!("../../assets/gltf/shard1.gltf"));
        model.set_frag_shader(include_str!("../../assets/shaders/no_texture_color.frag").to_string());
        model.set_vert_shader(include_str!("../../assets/shaders/no_texture.vert").to_string());
        model.set_scale(0.05, 0.05, 0.05);

        let red = 1.0;
        let green = fastrand::f32() * 0.5 + 0.5;
        let blue = 0.0;
        let col= [red, green, blue];

        Self {
            model: model,
            position: Location3D::new(0.0, 0.0, 0.0),
            velocity: Location3D::new(0.0, 0.0, 0.0),
            color: col,
        }
    }

    pub fn setup(&mut self, gl: &GL, width: f32, height: f32) {
        if self.model.is_ready_to_load() {
            self.model.set_shader_val("u_color".to_string(), self.color);
            self.model.setup_shader(&gl, width, height);
            self.model.load_textures(&gl);
            self.model.setup(&gl);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        // Update shard position and rotation based on velocity and delta_time
        self.velocity.y -= FIREBALL_SHARD_GRAVITY * (delta_time as f32) * 0.001; // Apply gravity to the y-velocity
        self.velocity.x *= FIREBALL_SHARD_DRAG; // Apply some damping to the x-velocity
        self.velocity.z *= FIREBALL_SHARD_DRAG * (delta_time as f32) * 0.001; // Apply some damping to the z-velocity

        self.position.x += self.velocity.x * (delta_time as f32);
        self.position.y += self.velocity.y * (delta_time as f32);
        self.position.z += self.velocity.z * (delta_time as f32);

        if self.position.y < 0.1 {
            self.position.y = 0.1;
            self.velocity.y *= -0.4; // Bounce with some energy loss
        }

        self.model.set_position(self.position.x, self.position.y, self.position.z);
    }

    pub fn render(&mut self, ctx: &GL, time: f64, camera: &Camera) {
        if self.model.is_ready_to_render() {
            self.model.render(ctx, time, camera);
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Location3D::new(x, y, z);
        self.model.set_position(x, y, z);
    }

    pub fn set_velocity(&mut self, x: f32, y: f32, z: f32) {
        self.velocity = Location3D::new(x, y, z);
    }
}