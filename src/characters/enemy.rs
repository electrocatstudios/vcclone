use web_sys::WebGlRenderingContext as GL;

use crate::{model::model::Model, utils::Location3D};
use crate::model::camera::Camera;
use crate::consts::*;

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
    destination: Location3D,
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
        
        let destination = Location3D {
            x: position.x,
            y: position.y,
            z: position.z,
        };

        Enemy {
            position: position,
            rotation: 0.0,
            head_model: head_model,
            body_model: body_model,
            left_arm_model: left_arm_model,
            right_arm_model: right_arm_model,
            left_shoe_model: left_shoe_model,
            right_shoe_model: right_shoe_model,
            destination: destination,
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

    fn set_new_destination(&mut self) {
        // For simplicity, we'll just move the enemy in a random direction within a certain range
        // let range = 5.0;
        self.destination.x = (rand::random::<f32>() * 4.0) - 2.0;//self.position.x + (rand::random::<f32>() - 0.5) * range;
        self.destination.z = (rand::random::<f32>() * 16.0) - 7.0;//self.position.z + (rand::random::<f32>() - 0.5) * range;
   
    }

    pub fn update(&mut self, delta: f64) {
        let target_angle = self.get_target_angle();//self.destination.z.atan2(self.destination.x);
        if self.position.x < self.destination.x + 0.1 && self.position.x > self.destination.x - 0.1 &&
           self.position.z < self.destination.z + 0.1 && self.position.z > self.destination.z - 0.1 {
            self.set_new_destination();
        } else if (self.rotation - target_angle).abs() > 0.01 {
            let angle_diff = (target_angle - self.rotation).abs();
            if angle_diff < ENEMY_ROTATION_SPEED {
                self.rotation = target_angle; // Snap to the target angle if we're close enough
            } else {
                self.rotation += ENEMY_ROTATION_SPEED * (target_angle - self.rotation).signum(); // Rotate towards the target angle
            }

        } else if self.position.x != self.destination.x || self.position.z != self.destination.z {
            self.rotation = target_angle; // Ensure the enemy is facing the destination
            self.position.x += ENEMY_MOVE_SPEED * target_angle.cos() * delta as f32; // Move towards the destination
            self.position.z += ENEMY_MOVE_SPEED * target_angle.sin() * delta as f32;
        } else {
            self.set_new_destination();
        }
        let rot = self.rotation + std::f32::consts::FRAC_PI_2;

        self.head_model.set_position(self.position.x, self.position.y + 0.185, self.position.z);
        self.head_model.set_rotation(0.0, rot, 0.0);

        self.body_model.set_position(self.position.x, self.position.y, self.position.z);
        self.body_model.set_rotation(0.0, rot, 0.0);

        self.left_arm_model.set_position(self.position.x + (0.15 * rot.cos()) , self.position.y, self.position.z + (0.15 * rot.sin()));
        self.left_arm_model.set_rotation(0.0, rot, 0.0);

        self.right_arm_model.set_position(self.position.x - (0.15 * rot.cos()), self.position.y, self.position.z - (0.15 * rot.sin()));
        self.right_arm_model.set_rotation(0.0, rot + std::f32::consts::PI, 0.0);

        let shoe_offset_x = (0.0625 * rot.cos()) + (0.055 * rot.sin());
        let shoe_offset_z =  (0.0625 * rot.sin()) - (0.055 * rot.cos()); // Distance from the center of the body to the shoe
        self.left_shoe_model.set_position(self.position.x + shoe_offset_x, self.position.y - 0.3, self.position.z + shoe_offset_z);
        self.left_shoe_model.set_rotation(0.0, rot + std::f32::consts::PI, 0.0);
        
        let shoe_offset_x = (0.0625 * rot.cos()) - (0.055 * rot.sin());
        let shoe_offset_z =  (0.0625 * rot.sin()) + (0.055 * rot.cos()); // Distance from the center of the body to the shoe
        self.right_shoe_model.set_position(self.position.x - shoe_offset_x, self.position.y - 0.3, self.position.z - shoe_offset_z);
        self.right_shoe_model.set_rotation(0.0, rot + std::f32::consts::PI, 0.0);
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

    pub fn get_location(&self) -> Location3D {
        self.position
    }

    pub fn get_destination(&self) -> Location3D {
        self.destination
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
    pub fn get_target_angle(&self) -> f32 {
        let diff_x: f32 = self.destination.x - self.position.x;
        let diff_z: f32 = self.destination.z - self.position.z;
        diff_z.atan2(diff_x)
    }
}


#[cfg(test)]
mod enemy_tests {
    use super::*;

    #[test]
    fn test_tan2() {
        // self.destination.z.atan2(self.destination.x)
        let loc_x: f32 = 0.0;
        let loc_z: f32 = 0.0;

        let dest_x: f32 = 1.0;
        let dest_z: f32 = 0.0;
        
        let diff_x: f32 = dest_x - loc_x;
        let diff_z: f32 = dest_z - loc_z;
        let angle: f32 = diff_z.atan2(diff_x);
        assert_eq!(angle, 0.0);

        let dest_z = 1.0;
        let dest_x = 0.0;
        let diff_x: f32 = dest_x - loc_x;
        let diff_z: f32 = dest_z - loc_z;
        let angle: f32 = diff_z.atan2(diff_x);
        assert_eq!(angle, std::f32::consts::FRAC_PI_2);


        // let position = Location3D { x: 0.0, y: 0.0, z: 0.0 };
        // let enemy = Enemy::new(position);
        // assert_eq!(enemy.get_location(), position);
        // assert_eq!(enemy.get_rotation(), 0.0);
    }

    // #[test]
    // fn test_enemy_movement() {
    //     let mut enemy = Enemy::new(Location3D { x: 0.0, y: 0.0, z: 0.0 });
    //     enemy.destination = Location3D { x: 1.0, y: 0.0, z: 1.0 };
    //     enemy.update(1000.0); // Simulate a large delta to ensure movement
    //     let new_location = enemy.get_location();
    //     assert!(new_location.x > 0.0 && new_location.z > 0.0); // Enemy should have moved towards the destination
    // }
}