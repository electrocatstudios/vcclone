use web_sys::WebGlRenderingContext as GL;

use crate::{model::model::Model, utils::Location3D};
use crate::model::camera::Camera;

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
    shards: Vec<Model>, // For explosion shards
}

impl Firebolt {
    pub fn new(location: Location3D, velocity: (f32, f32, f32)) -> Self {
        let mut model = Model::new("firebolt".to_string()); // Load the firebolt model here
        model.set_gltf(include_str!("../../assets/gltf/firebolt.gltf"));
        model.set_frag_shader(include_str!("../../assets/shaders/no_texture.frag").to_string());
        model.set_vert_shader(include_str!("../../assets/shaders/no_texture.vert").to_string());
        model.set_rotation(0.0, -std::f32::consts::PI/2.0, 0.0);

        let mut shards = Vec::new();
        
        let mut shard = Model::new("shard_1".to_string());
        shard.set_gltf(include_str!("../../assets/gltf/shard1.gltf"));
        shard.set_frag_shader(include_str!("../../assets/shaders/no_texture.frag").to_string());
        shard.set_vert_shader(include_str!("../../assets/shaders/no_texture.vert").to_string());
        shard.set_scale(0.05, 0.05, 0.05);

        shards.push(shard);
        
        let mut shard = Model::new("shard_2".to_string());
        shard.set_gltf(include_str!("../../assets/gltf/shard2.gltf"));
        shard.set_frag_shader(include_str!("../../assets/shaders/no_texture.frag").to_string());
        shard.set_vert_shader(include_str!("../../assets/shaders/no_texture.vert").to_string());
        shard.set_scale(0.05, 0.05, 0.05);

        shards.push(shard);
        
        let mut shard = Model::new("shard_3".to_string());
        shard.set_gltf(include_str!("../../assets/gltf/shard3.gltf"));
        shard.set_frag_shader(include_str!("../../assets/shaders/no_texture.frag").to_string());
        shard.set_vert_shader(include_str!("../../assets/shaders/no_texture.vert").to_string());
        shard.set_scale(0.05, 0.05, 0.05);

        shards.push(shard);

        
        let mut shard = Model::new("shard_4".to_string());
        shard.set_gltf(include_str!("../../assets/gltf/shard2.gltf"));
        shard.set_frag_shader(include_str!("../../assets/shaders/no_texture.frag").to_string());
        shard.set_vert_shader(include_str!("../../assets/shaders/no_texture.vert").to_string());
        shard.set_scale(0.05, 0.05, 0.05);

        shards.push(shard);

        let mut shard = Model::new("shard_5".to_string());
        shard.set_gltf(include_str!("../../assets/gltf/shard1.gltf"));
        shard.set_frag_shader(include_str!("../../assets/shaders/no_texture.frag").to_string());
        shard.set_vert_shader(include_str!("../../assets/shaders/no_texture.vert").to_string());
        shard.set_scale(0.05, 0.05, 0.05);

        shards.push(shard);

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
            if shard.is_ready_to_load() {
                shard.setup_shader(&gl, width, height);
                shard.load_textures(&gl);
                shard.setup(&gl);
            }
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
                let pos = shard.position;
                shard.set_position(pos.x, pos.y - ((delta as f32) * 0.001), pos.z);

                // shard.set_rotation(
                //     shard.rotation.x + (self.cooldown * 0.01) as f32,
                //     shard.rotation.y + (self.cooldown * 0.01) as f32,
                //     shard.rotation.z + (self.cooldown * 0.01) as f32,
                // );
            }
        }

        
    }

    pub fn get_location(&self) -> &Location3D {
        &self.location
    }

    pub fn render(&mut self, ctx: &GL, time: f64, camera: &Camera) {
        if self.state == FireboltState::Flying {
             if self.model.is_ready_to_render() {
                self.model.render(ctx, time, camera);
            }
        } else if self.state == FireboltState::Exploding {
             for shard in self.shards.iter_mut() {
                 if shard.is_ready_to_render() {
                    shard.render(ctx, time, camera);
                }
             }
        }
        // if self.model.is_ready_to_render() {
        //     self.model.render(ctx, time, camera);
        // }

    }

    pub fn destroy(&mut self) {
        gloo_console::log!("Firebolt hit something and is now exploding!");
        self.state = FireboltState::Exploding;

        let mut x = -0.2;
        let mut y = 0.2;

        for shard in self.shards.iter_mut() {
            shard.set_position(self.location.x + x, self.location.y+y, self.location.z);
            x += 0.1;
            y -= 0.1;
        }

    }

    pub fn is_expired(&self) -> bool {
        self.state == FireboltState::Expired
    }
}