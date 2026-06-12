use web_sys::WebGlRenderingContext as GL;

use crate::{model::model::Model, utils::Location3D};
use crate::model::camera::Camera;

#[derive(PartialEq)]
enum EnemyState {
    Idle,
}

pub struct Enemy {
    position: Location3D,
    rotation: f32,
    head_model: Model,
    body_model: Model,
    left_arm_model: Model,
    right_arm_model: Model,
    left_shoe_model: Model,
    right_shoe_model: Model,
    state: EnemyState,
}

impl Enemy {
    pub fn new(position: Location3D) -> Self {
        let mut head_model = Model::new("enemy_head".to_string()); // Load the head model here
        head_model.set_gltf(include_str!("../../assets/gltf/enemy_head.gltf"));
        head_model.set_frag_shader(include_str!("../../assets/shaders/no_texture.frag").to_string());
        head_model.set_vert_shader(include_str!("../../assets/shaders/no_texture.vert").to_string());
        head_model.set_rotation(0.0, 0.0, 0.0);
        head_model.set_scale(0.25, 0.25, 0.25); // Scale up the head model to fit the character

        let mut body_model = Model::new("enemy_body".to_string()); // Load the body model here
        body_model.set_gltf(include_str!("../../assets/gltf/enemy_body.gltf"));
        body_model.set_frag_shader(include_str!("../../assets/shaders/no_texture.frag").to_string());
        body_model.set_vert_shader(include_str!("../../assets/shaders/no_texture.vert").to_string());
        body_model.set_rotation(0.0, 0.0, 0.0);
        body_model.set_scale(0.25, 0.25, 0.25); // Scale up the body model to fit the character
        
        let mut left_arm_model = Model::new("enemy_left_arm".to_string()); // Load the left arm model here
        left_arm_model.set_gltf(include_str!("../../assets/gltf/enemy_arm.gltf"));
        left_arm_model.set_frag_shader(include_str!("../../assets/shaders/no_texture.frag").to_string());
        left_arm_model.set_vert_shader(include_str!("../../assets/shaders/no_texture.vert").to_string());   
        left_arm_model.set_rotation(0.0, 0.0, 0.0);
        left_arm_model.set_scale(0.25, 0.25, 0.25); // Scale up the arm model to fit the character

        let mut right_arm_model = Model::new("enemy_right_arm".to_string()); // Load the left arm model here
        right_arm_model.set_gltf(include_str!("../../assets/gltf/enemy_arm.gltf"));
        right_arm_model.set_frag_shader(include_str!("../../assets/shaders/no_texture.frag").to_string());
        right_arm_model.set_vert_shader(include_str!("../../assets/shaders/no_texture.vert").to_string());   
        right_arm_model.set_rotation(0.0, std::f32::consts::PI, 0.0);
        right_arm_model.set_scale(0.25, 0.25, 0.25); // Scale up the arm model to fit the character


        let mut left_shoe_model = Model::new("enemy_left_shoe".to_string()); // Load the left shoe model here
        left_shoe_model.set_gltf(include_str!("../../assets/gltf/enemy_shoe.gltf"));
        left_shoe_model.set_frag_shader(include_str!("../../assets/shaders/no_texture.frag").to_string());
        left_shoe_model.set_vert_shader(include_str!("../../assets/shaders/no_texture.vert").to_string());
        left_shoe_model.set_rotation(0.0, 0.0, 0.0);
        left_shoe_model.set_scale(0.375, 0.5, 0.5); // Scale down the shoe model to fit the character

        let mut right_shoe_model = Model::new("enemy_right_shoe".to_string()); // Load the right shoe model here
        right_shoe_model.set_gltf(include_str!("../../assets/gltf/enemy_shoe.gltf"));
        right_shoe_model.set_frag_shader(include_str!("../../assets/shaders/no_texture.frag").to_string());
        right_shoe_model.set_vert_shader(include_str!("../../assets/shaders/no_texture.vert").to_string());
        right_shoe_model.set_rotation(0.0, 0.0, 0.0); 
        right_shoe_model.set_scale(0.375, 0.5, 0.5); // Scale down the shoe model to fit the character
        
        Enemy {
            position: position,
            rotation: 0.0,
            head_model: head_model,
            body_model: body_model,
            left_arm_model: left_arm_model,
            right_arm_model: right_arm_model,
            left_shoe_model: left_shoe_model,
            right_shoe_model: right_shoe_model,
            state: EnemyState::Idle,
        }
    }

    pub fn setup(&mut self, gl: &GL, width: f32, height: f32) {
        if self.head_model.is_ready_to_load() {
            self.head_model.setup_shader(&gl, width, height);
            self.head_model.load_textures(&gl);
            self.head_model.setup(&gl);
        }

        if self.body_model.is_ready_to_load() {
            self.body_model.setup_shader(&gl, width, height);
            self.body_model.load_textures(&gl);
            self.body_model.setup(&gl);
        }

        if self.left_arm_model.is_ready_to_load() {
            self.left_arm_model.setup_shader(&gl, width, height);
            self.left_arm_model.load_textures(&gl);
            self.left_arm_model.setup(&gl);
        }

        if self.right_arm_model.is_ready_to_load() {
            self.right_arm_model.setup_shader(&gl, width, height);
            self.right_arm_model.load_textures(&gl);
            self.right_arm_model.setup(&gl);
        }

        if self.left_shoe_model.is_ready_to_load() {
            self.left_shoe_model.setup_shader(&gl, width, height);
            self.left_shoe_model.load_textures(&gl);
            self.left_shoe_model.setup(&gl);
        }

        if self.right_shoe_model.is_ready_to_load() {
            self.right_shoe_model.setup_shader(&gl, width, height);
            self.right_shoe_model.load_textures(&gl);
            self.right_shoe_model.setup(&gl);
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.rotation += 0.001 * delta as f32; // Rotate the enemy over time

        self.position.x -= 0.01 * self.rotation.cos(); // Move the enemy in a circular pattern
        self.position.z -= 0.01 * self.rotation.sin();

        self.head_model.set_position(self.position.x, self.position.y + 0.185, self.position.z);
        self.head_model.set_rotation(0.0, self.rotation, 0.0);

        self.body_model.set_position(self.position.x, self.position.y, self.position.z);
        self.body_model.set_rotation(0.0, self.rotation, 0.0);

        self.left_arm_model.set_position(self.position.x + (0.15 * self.rotation.cos()) , self.position.y, self.position.z + (0.15 * self.rotation.sin()));
        self.left_arm_model.set_rotation(0.0, self.rotation, 0.0);

        self.right_arm_model.set_position(self.position.x - (0.15 * self.rotation.cos()), self.position.y, self.position.z - (0.15 * self.rotation.sin()));
        self.right_arm_model.set_rotation(0.0, self.rotation + std::f32::consts::PI, 0.0);

        let shoe_offset_x = (0.0625 * self.rotation.cos()) + (0.055 * self.rotation.sin());
        let shoe_offset_z =  (0.0625 * self.rotation.sin()) - (0.055 * self.rotation.cos()); // Distance from the center of the body to the shoe
        self.left_shoe_model.set_position(self.position.x + shoe_offset_x, self.position.y - 0.3, self.position.z + shoe_offset_z);
        self.left_shoe_model.set_rotation(0.0, self.rotation + std::f32::consts::PI, 0.0);
        
        let shoe_offset_x = (0.0625 * self.rotation.cos()) - (0.055 * self.rotation.sin());
        let shoe_offset_z =  (0.0625 * self.rotation.sin()) + (0.055 * self.rotation.cos()); // Distance from the center of the body to the shoe
        self.right_shoe_model.set_position(self.position.x - shoe_offset_x, self.position.y - 0.3, self.position.z - shoe_offset_z);
        self.right_shoe_model.set_rotation(0.0, self.rotation + std::f32::consts::PI, 0.0);
    }

    pub fn render(&mut self, gl: &GL, time: f64, camera: &Camera) {
        if self.head_model.is_ready_to_render() {
            self.head_model.render(gl, time, camera);
        }
        if self.body_model.is_ready_to_render() {
            self.body_model.render(gl, time, camera);
        }
        
        if self.left_arm_model.is_ready_to_render() {
            self.left_arm_model.render(gl, time, camera);
        }
        if self.right_arm_model.is_ready_to_render() {
            self.right_arm_model.render(gl, time, camera);
        }
        if self.left_shoe_model.is_ready_to_render() {
            self.left_shoe_model.render(gl, time, camera);
        }
        if self.right_shoe_model.is_ready_to_render() {
            self.right_shoe_model.render(gl, time, camera);
        }

    }

}