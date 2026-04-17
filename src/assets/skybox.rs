use base64::{Engine as _, engine::general_purpose};
use web_sys::WebGlRenderingContext as GL;

// use crate::model::;
use crate::model::{camera::Camera, model::Model};

pub struct Skybox {
    pub model: Model,
}

impl Skybox {
    pub fn new() -> Self {
        let mut model = Model::new("skybox".to_string());
        model.set_gltf(include_str!("../../assets/gltf/skybox.gltf"));
        model.set_frag_shader(include_str!("../../assets/shaders/cube_texture.frag").to_string());
        model.set_vert_shader(include_str!("../../assets/shaders/cube_texture.vert").to_string());

        let tex_b64 = general_purpose::STANDARD.encode(include_bytes!("../../assets/texture/dice_skin.png"));
        let data_url = format!("data:image/png;base64,{}", tex_b64);
        model.set_texture_base64(data_url);
        model.set_scale(5.0, 4.0, 20.0);
        model.set_position(0.0, 2.0, 0.0);
        model.set_rotation(0.0, 0.0, 0.0);

        Self {
            model: model,
        }
    }

    pub fn update(&mut self, _delta: f64, gl: &GL, width: f32, height: f32) {

        if self.model.is_ready_to_load() {
            self.model.setup_shader(&gl, width, height);
            self.model.load_textures(&gl);
            self.model.setup(&gl);
        }

        // self.model.update(delta);
    }

    pub fn render(&mut self, gl: &GL, time: f64, camera: &Camera) {
        if self.model.is_ready_to_render() {
            self.model.render(gl, time, camera);
        }
    }
}