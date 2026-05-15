use web_sys::WebGlRenderingContext as GL;

use crate::{model::model::Model, utils::Location3D};
use crate::model::camera::Camera;
use super::shard::Shard;

#[derive(PartialEq)]
enum FireboltState {
    Flying,
    Exploding,
    Expired
}

const FIREBOLT_LIFETIME: f64 = 1500.0; // Firebolt explodes for 1.5 seconds before expiring

pub struct Firebolt {
    location: Location3D,
    velocity: (f32, f32, f32),
    model: Model,
    state: FireboltState,
    cooldown: f64,
    shards: Vec<Shard>, // For explosion shards
}

impl Firebolt {
    pub fn new(location: Location3D, velocity: (f32, f32, f32)) -> Self {
        let mut model = Model::new("firebolt".to_string()); // Load the firebolt model here
        model.set_gltf(include_str!("../../assets/gltf/firebolt.gltf"));
        model.set_frag_shader(include_str!("../../assets/shaders/no_texture.frag").to_string());
        model.set_vert_shader(include_str!("../../assets/shaders/no_texture.vert").to_string());
        model.set_rotation(0.0, -std::f32::consts::PI/2.0, 0.0);

        let mut shards = Vec::new();
        let num_shards = fastrand::usize(5..8);
        for i in 0..num_shards {
            let name = format!("shard_{}", i + 1);
            shards.push(Shard::new(name));
        }
        
        Firebolt {
            location,
            velocity,
            model: model, // Initialize the model here
            state: FireboltState::Flying,
            cooldown: FIREBOLT_LIFETIME,
            shards: shards,
        }
    }

    pub fn setup(&mut self, gl: &GL, width: f32, height: f32) {
        if self.model.is_ready_to_load() {
            self.model.setup_shader(&gl, width, height);
            self.model.load_textures(&gl);
            self.model.setup(&gl);
        }

        for shard in self.shards.iter_mut() {
            shard.setup(gl, width, height);
        }
    }

    pub fn update(&mut self, delta: f64) {

        if self.state == FireboltState::Flying {
            self.location.x += self.velocity.0 * delta as f32;
            self.location.y += self.velocity.1 * delta as f32;
            self.location.z += self.velocity.2 * delta as f32;

            self.model.set_position(self.location.x, self.location.y, self.location.z);
            
            if self.location.z > -4.0 {
                self.destroy();
            }

        } else if self.state == FireboltState::Exploding {
            self.cooldown -= delta;
            self.state = if self.cooldown <= 0.0 {
                FireboltState::Expired
            } else {
                FireboltState::Exploding
            };

            for shard in self.shards.iter_mut() {
                shard.update(delta);
            }
        }

        
    }

    pub fn _get_location(&self) -> &Location3D {
        &self.location
    }

    pub fn render(&mut self, ctx: &GL, time: f64, camera: &Camera) {
        if self.state == FireboltState::Flying {
             if self.model.is_ready_to_render() {
                self.model.render(ctx, time, camera);
            }
        } else if self.state == FireboltState::Exploding {
             for shard in self.shards.iter_mut() {
                shard.render(ctx, time, camera);
             }
        }

    }

    pub fn destroy(&mut self) {
        gloo_console::log!("Firebolt hit something and is now exploding!");
        self.state = FireboltState::Exploding;

        for shard in self.shards.iter_mut() {
            let pos_x = self.location.x + (fastrand::f32() * 0.2 - 0.1); // Random position offset between -0.1 and 0.1
            let pos_y = self.location.y + (fastrand::f32() * 0.2 - 0.1);
            let pos_z = self.location.z;

            shard.set_position(pos_x, pos_y, pos_z);
            
            let vel_x = fastrand::f32() * 0.01 - 0.005; // Random velocity between -0.01 and 0.01
            let vel_y = fastrand::f32() * 0.004;
            let vel_z = fastrand::f32() * 0.02 - 0.01;
            shard.set_velocity(vel_x, vel_y, vel_z);
        }
    }

    pub fn is_expired(&self) -> bool {
        self.state == FireboltState::Expired
    }
}