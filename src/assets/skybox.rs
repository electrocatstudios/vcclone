use base64::{Engine as _, engine::general_purpose};
use web_sys::WebGlRenderingContext as GL;

// use crate::model::;
use crate::model::{camera::Camera, model::Model};

pub struct Skybox {
    pub model: Model,
    pub night_sky: Model,
}

impl Skybox {
    pub fn new() -> Self {
        let mut model = Model::new("skybox".to_string());
        model.set_gltf(include_str!("../../assets/gltf/skybox.gltf"));
        model.set_frag_shader(include_str!("../../assets/shaders/cube_texture.frag").to_string());
        model.set_vert_shader(include_str!("../../assets/shaders/cube_texture.vert").to_string());
        model.set_use_transparency(true);
        
        let tex_b64 = general_purpose::STANDARD.encode(include_bytes!("../../assets/texture/skybox.png"));
        let data_url = format!("data:image/png;base64,{}", tex_b64);
        model.set_texture_base64(data_url);
        model.set_scale(5.0, 4.0, 20.0);
        model.set_position(0.0, 2.0, 0.0);
        model.set_rotation(0.0, 0.0, 0.0);

        let mut night_sky = Model::new("night_sky".to_string());
        night_sky.set_gltf(include_str!("../../assets/gltf/plane.gltf"));
        night_sky.set_frag_shader(include_str!("../../assets/shaders/cube_texture.frag").to_string());
        night_sky.set_vert_shader(include_str!("../../assets/shaders/cube_texture.vert").to_string());
        
        let tex_b64 = general_purpose::STANDARD.encode(include_bytes!("../../assets/texture/starry_sky.png"));
        let data_url = format!("data:image/png;base64,{}", tex_b64);
        night_sky.set_texture_base64(data_url);
        night_sky.set_scale(20.0, 40.0, 20.0);
        night_sky.set_position(0.0, 8.0, 18.0);
        night_sky.set_rotation(-std::f32::consts::FRAC_PI_2, 0.0, 0.0);
        
        Self {
            model: model,
            night_sky: night_sky,
        }
    }

    pub fn update(&mut self, _delta: f64, gl: &GL, width: f32, height: f32) {

        if self.model.is_ready_to_load() {
            self.model.setup_shader(&gl, width, height);
            self.model.load_textures(&gl);
            self.model.setup(&gl);
        }

        if self.night_sky.is_ready_to_load() {
            self.night_sky.setup_shader(&gl, width, height);
            self.night_sky.load_textures(&gl);
            self.night_sky.setup(&gl);
        }
    }

    pub fn render(&mut self, gl: &GL, time: f64, camera: &Camera) {
        if self.night_sky.is_ready_to_render() {
            self.night_sky.render(gl, time, camera);
        }
        if self.model.is_ready_to_render() {
            self.model.render(gl, time, camera);
        }
        
    }
}