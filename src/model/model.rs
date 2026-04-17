use std::{collections::HashMap, rc::Rc};

use euclid::{Transform3D, Vector3D};

use gltf_json::mesh::Semantic;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlImageElement, WebGlProgram, WebGlRenderingContext as GL};

use super::camera::Camera;

use crate::utils;

pub struct Model {
    pub name: String,
    pub _gltf_name: Option<String>,
    pub _frag_shader_name: Option<String>,
    pub _vert_shader_name: Option<String>,
    pub gltf: Option<gltf_json::Root>,
    pub matrix: Transform3D<f32, (), ()>,
    pub position: Vector3D<f32, ()>,
    pub rotation: Vector3D<f32, ()>,
    pub scale: Vector3D<f32, ()>,
    pub base_scale: Vector3D<f32, ()>,
    pub normal_buffer: Option<web_sys::WebGlBuffer>,
    pub offset_stride: HashMap::<String, (i32, i32)>,
    pub pos_buffer: Option<web_sys::WebGlBuffer>,
    pub texcoord_buffer: Option<web_sys::WebGlBuffer>,
    pub color_buffer: Option<web_sys::WebGlBuffer>,
    pub texture_buffer: Option<web_sys::WebGlTexture>,
    pub time_location: Option<web_sys::WebGlUniformLocation>,
    pub _position_location: Option<u32>,
    pub shader_program: Option<WebGlProgram>,
    pub poly_count: usize,
    vertex_content: Option<String>,
    fragment_content: Option<String>,
    texture_name: Option<String>,
    is_loaded: bool,
    pub use_transparency: bool,
    pub color: Option<[f32; 3]>,
    texture_loaded: bool,
    vars: HashMap<String, [f32; 3]>,
}

// const TEXTURE_1: &str = "/api/texture/noise.png";

impl Model {
    pub fn new(name: String) -> Model {
        Model {
            name: name,
            _gltf_name: None,
            _frag_shader_name: None,
            _vert_shader_name: None,
            gltf: None,
            matrix: Transform3D::identity(),
            position: Vector3D::new(0.0, 0.0, 0.0),
            rotation: Vector3D::new(0.0, 0.0, 0.0),
            scale: Vector3D::new(1.0, 1.0, 1.0),
            base_scale: Vector3D::new(1.0, 1.0, 1.0),
            normal_buffer: None,
            pos_buffer: None,
            offset_stride: HashMap::<String, (i32, i32)>::new(),
            texcoord_buffer: None,
            color_buffer: None,
            texture_buffer: None,
            _position_location: None,
            time_location: None,
            shader_program: None,
            poly_count: 0,
            vertex_content: None,
            fragment_content: None,
            texture_name: None,
            is_loaded: false,
            use_transparency: false,
            color: None,
            texture_loaded: false,
            vars: HashMap::new(),
        }
    }

    pub fn set_frag_shader(&mut self, content: String) {
        self.fragment_content = Some(content);
    }
    pub fn set_vert_shader(&mut self, content: String) {
        self.vertex_content = Some(content);
    }
    pub fn set_gltf(&mut self, gltf_str: &str) {
        // gloo_console::log!("Setting glTF content for model: ", gltf_str.clone());
        let gltf_json: gltf_json::Root = serde_json::from_str(gltf_str).unwrap();
        self.gltf = Some(gltf_json);
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        // gloo_console::log!("Applying scale factor: ", self.scale_factor, x * self.scale_factor * 0.1, y);
        self.scale = Vector3D::new(x * self.base_scale.x, y * self.base_scale.y, z * self.base_scale.z);
    }

    pub fn set_base_scale(&mut self, x: f32, y: f32, z: f32) {
        self.base_scale = Vector3D::new(x, y, z);
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Vector3D::new(x, y, z);
        self.matrix = Transform3D::translation(x, y, z);
    }

    pub fn set_texture_base64(&mut self, base64_content: String) {
        // gloo_console::log!("Setting texture content for model: ", base64_content.clone());   
        self.texture_name = Some(base64_content);
        self.texture_loaded = false;
    }

    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.rotation = Vector3D::new(x, y, z);
        // self.matrix = self.matrix.then_rotate(1.0, 0.0, 0.0, euclid::Angle::radians(x));
        // self.matrix = self.matrix.then_rotate(0.0, 1.0, 0.0, euclid::Angle::radians(y));
        // self.matrix = self.matrix.then_rotate(0.0, 0.0, 1.0, euclid::Angle::radians(z));
        // gloo_console::log!("Rotation set to: ", self._rotation.y);
    }

    pub fn _set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = Some([r, g, b]);
    }

    pub fn set_shader_val(&mut self, var_name: String, vals: [f32; 3]) {
        self.vars.insert(var_name, vals);
    }

    pub fn is_ready_to_load(&self) -> bool {
        !self.is_loaded && self.gltf.is_some() && self.fragment_content.is_some() && self.vertex_content.is_some()
    }
    pub fn is_ready_to_render(&self) -> bool {
        self.is_loaded
    }

    pub fn setup_shader(&mut self, gl: &GL, width: f32, height: f32) { // TODO: Add shader name so each one can have it's own
        let vert_code = match &self.vertex_content {
            Some(code) => code,
            None => {
                gloo_console::log!("No vertex shader code found");
                return;
            }
        };
        let frag_code = match &self.fragment_content {
            Some(code) => code,
            None => {
                gloo_console::log!("No fragment shader code found");
                return;
            }
        };
        self.is_loaded = true;
        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, &vert_code);
        gl.compile_shader(&vert_shader);

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, &frag_code);
        gl.compile_shader(&frag_shader);

        let shader_program: WebGlProgram = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));

        // Attach the position vector as an attribute for the GL context.
        let canvassize = gl.get_uniform_location(&shader_program, "u_screensize");
        gl.uniform2f(canvassize.as_ref(), width, height);

        self.time_location = gl.get_uniform_location(&shader_program, "u_time");
        gl.uniform1f(self.time_location.as_ref() , 1.0);

        self.shader_program = Some(shader_program);
        // gloo_console::log!("Shader program set up for model: ", self.name.clone());
    }

    pub fn setup(&mut self, gl: &GL) {
        let gltf = match &self.gltf {
            Some(gltf) => gltf,
            None => {
                gloo_console::log!("Trying to load before gltf has been set");
                return;
            }
        };
        // TODO: loop through all meshes and perform this - rather than first one
        let mesh = gltf.meshes.get(0).unwrap();
        mesh.primitives.iter().for_each(|primitive| {
            /* Position Buffer */
            match utils::get_data_from_buffer(primitive, &gltf, Semantic::Positions, false){
                Ok(buffer) => {
                    let conv_buffer = utils::get_f32_buffer_from_u8(buffer.buffer);
                    // gloo_console::log!("dims for model: ", buffer.triangle_count, buffer._byte_offset, buffer._byte_stride);
                    self.poly_count = buffer.triangle_count as usize;
                    let vertex_buffer = gl.create_buffer().unwrap();
                    let verts = js_sys::Float32Array::from(conv_buffer.as_slice());

                    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
                    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

                    match &self.shader_program {
                        Some(sp) => {
                            self.offset_stride.insert("position".to_string(), (buffer._byte_offset as i32, buffer._byte_stride as i32));
                            let att = gl.get_attrib_location(sp, "a_position");
                            gl.vertex_attrib_pointer_with_i32(att as u32, 3, GL::FLOAT, false, buffer._byte_stride as i32, buffer._byte_offset as i32);
                            gl.enable_vertex_attrib_array(att as u32);
                        },
                        None => {
                            gloo_console::log!("No shader program found while setting a_position");
                        }
                    }
                    gl.bind_buffer(GL::ARRAY_BUFFER, None);
                    self.pos_buffer = Some(vertex_buffer);
                },
                Err(err) => {
                    gloo_console::log!("Error getting position buffer: ", err);
                    return; // Can't continue witout positions
                }
            };
            /* End of Position Buffer */

            /* Normal Buffer */
            match utils::get_data_from_buffer(primitive, &gltf, Semantic::Normals,false){
                Ok(buffer) => {
                    let conv_buffer = utils::get_f32_buffer_from_u8(buffer.buffer);
                    let vertex_buffer = gl.create_buffer().unwrap();
                    let verts = js_sys::Float32Array::from(conv_buffer.as_slice());
                    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
                    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);
                    match &self.shader_program {
                        Some(sp) => {
                            self.offset_stride.insert("normal".to_string(), (buffer._byte_offset as i32, buffer._byte_stride as i32));
                            let att = gl.get_attrib_location(sp, "a_normal");
                            gl.vertex_attrib_pointer_with_i32(att as u32, 3, GL::FLOAT, false, buffer._byte_stride as i32, buffer._byte_offset as i32);
                            gl.enable_vertex_attrib_array(att as u32);
                        },
                        None => {
                            gloo_console::log!("No shader program found while setting a_position");
                        }
                    }
                    self.normal_buffer = Some(vertex_buffer);
                    gl.bind_buffer(GL::ARRAY_BUFFER, None);
                },
                Err(err) => {
                    gloo_console::log!("Error getting normal buffer: ", err);
                }
            };
            /* End Normal Buffer */

            /* Tex_coord Buffer */
            // TODO: Support multiple tex coords
            match utils::get_data_from_buffer(primitive, &gltf, Semantic::TexCoords(0), false){
                Ok(buffer) => {
                    let conv_buffer = utils::get_f32_buffer_from_u8(buffer.buffer);
                    let vertex_buffer = gl.create_buffer().unwrap();
                    let verts = js_sys::Float32Array::from(conv_buffer.as_slice());
                    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
                    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

                    match &self.shader_program {
                        Some(sp) => {                            
                            self.offset_stride.insert("texcoord".to_string(), (buffer._byte_offset as i32, buffer._byte_stride as i32));
                            let att = gl.get_attrib_location(sp, "a_texcoord");
                            gl.vertex_attrib_pointer_with_i32(att as u32, 2, GL::FLOAT, false, buffer._byte_stride as i32, buffer._byte_offset as i32);
                            gl.enable_vertex_attrib_array(att as u32);
                        },
                        None => {}
                    }
                    self.texcoord_buffer = Some(vertex_buffer);
                    gl.bind_buffer(GL::ARRAY_BUFFER, None);
                },
                Err(err) => {
                    gloo_console::log!("Error getting tex coord buffer: ", err);
                }
            };
            /* End Tex_coord Buffer */

            /* Color Buffer */
            // TODO: Support multiple Color buffers
            match utils::get_data_from_buffer(primitive, &gltf, Semantic::Colors(0), false){
                Ok(buffer) => {
                    let conv_buffer = utils::get_f32_buffer_from_u8(buffer.buffer);

                    let vertex_buffer = gl.create_buffer().unwrap();
                    let verts = js_sys::Float32Array::from(conv_buffer.as_slice());
                    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
                    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);
                    match &self.shader_program {
                        Some(sp) => {
                            let att = gl.get_attrib_location(sp, "a_color");
                            self.offset_stride.insert("color".to_string(), (buffer._byte_offset as i32, buffer._byte_stride as i32));
                            gl.vertex_attrib_pointer_with_i32(att as u32, 3, GL::FLOAT, false,buffer._byte_stride as i32, buffer._byte_offset as i32);
                            gl.enable_vertex_attrib_array(att as u32);
                        },
                        None => {}
                    }
                    self.color_buffer = Some(vertex_buffer);
                    gl.bind_buffer(GL::ARRAY_BUFFER, None);
                },
                Err(err) => {
                    gloo_console::log!("Error getting tex coord buffer: ", err);
                }
            };
            /* End Color Buffer */

            // Other buffers
        });
    }

    pub fn load_textures(&mut self, gl: &GL) {
        let texture_name = match &self.texture_name {
            Some(name) => name,
            None => {
                // gloo_console::log!("No texture name found for model: ", self.name.clone());
                return;
            }
        };

        // gloo_console::log!("Loading texture for model: ", texture_name.clone());
        // Setup the texture
        // based on https://snoozetime.github.io/2019/12/19/webgl-texture.html
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

        let image: HtmlImageElement = HtmlImageElement::new().unwrap();
        let imgrc = Rc::new(image.clone());

        {
            let image = imgrc.clone();
            let texture = texture.clone();
            let gl = Rc::new(gl.clone());
            self.texture_buffer = Some(texture.clone());

            let a = Closure::wrap(Box::new(move || {
                gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

                let _ = gl.tex_image_2d_with_u32_and_u32_and_image(
                    GL::TEXTURE_2D,
                    0,
                    GL::RGBA.try_into().unwrap(),
                    GL::RGBA.try_into().unwrap(),
                    GL::UNSIGNED_BYTE,
                    &image,
                );

                // different from webgl1 where we need the pic to be power of 2
                gl.generate_mipmap(GL::TEXTURE_2D);
            }) as Box<dyn FnMut()>);

            imgrc.set_onload(Some(a.as_ref().unchecked_ref()));

            // Normally we'd store the handle to later get dropped at an appropriate
            // time but for now we want it to be a global handler so we use the
            // forget method to drop it without invalidating the closure. Note that
            // this is leaking memory in Rust, so this should be done judiciously!
            a.forget();
        }
        // gloo_console::log!("Loading texture: ", texture_name.clone());
        image.set_src(texture_name);
        self.texture_loaded = true;
    }

    pub fn _update(&mut self, _time: f32) {

        // Undo position tranform before applying rotations
        // self.matrix = self.matrix.then_translate([-self.position.x, -self.position.y, -self.position.z].into());

        // // Do the rotation
        // if self._rotation.y != 0.0 {
        //     gloo_console::log!("Applying rotation: ", self._rotation.y);
        // }
        // self.matrix = self.matrix.then_rotate(1.0, 0.0, 0.0,Angle{radians: self._rotation.x});
        // self.matrix = self.matrix.then_rotate(0.0, 1.0, 0.0,Angle{radians: self._rotation.y});
        // self.matrix = self.matrix.then_rotate(0.0, 0.0, 1.0,Angle{radians: self._rotation.z});

        // // Calculation new translation position        
        // // Apply the new translation
        // self.matrix = self.matrix.then_translate([self.position.x, self.position.y, self.position.z].into()); 
    }

    pub fn render(&mut self, gl: &GL, time: f64, camera: &Camera) {

        if !self.texture_loaded {
            self.load_textures(gl);
        }

        gl.use_program(self.shader_program.as_ref());

        let scale_loc = gl.get_uniform_location(self.shader_program.as_ref().unwrap(), "u_scale");
        gl.uniform3f(
            scale_loc.as_ref(),
            self.scale.x,
            self.scale.y,
            self.scale.z,
        );

        let rot_loc = gl.get_uniform_location(self.shader_program.as_ref().unwrap(), "u_rotation");
        gl.uniform3f(
            rot_loc.as_ref(),
            self.rotation.x,
            self.rotation.y,
            self.rotation.z,
        );
        // gloo_console::log!(format!("Model Rotation Y: {}", self.rotation.y));

        // Update uniforms
        let canvassize = gl.get_uniform_location(&self.shader_program.as_mut().unwrap(), "u_screensize");
        gl.uniform2f(canvassize.as_ref(), camera.width, camera.height);

        let proj_loc = gl.get_uniform_location(&self.shader_program.as_mut().unwrap(), "u_projection");
        let vals: [f32; 16] = camera.projection.to_array();
        gl.uniform_matrix4fv_with_f32_array(proj_loc.as_ref() , false, &vals);

        let view_loc = gl.get_uniform_location(&self.shader_program.as_mut().unwrap(), "u_view");
        let vals: [f32; 16] = camera.view.to_array();
        gl.uniform_matrix4fv_with_f32_array(view_loc.as_ref() , false, &vals);

        let model_loc = gl.get_uniform_location(&self.shader_program.as_mut().unwrap(), "u_model");
        let vals: [f32; 16] = self.matrix.to_array();
        gl.uniform_matrix4fv_with_f32_array(model_loc.as_ref() , false, &vals);

        
        // Add vars to shader
        for (var_name, var_value) in &self.vars {
            let loc = gl.get_uniform_location(&self.shader_program.as_mut().unwrap(), var_name);
            gl.uniform3f(loc.as_ref(), var_value[0], var_value[1], var_value[2]);
        }

        if let Some(ref pos_buffer) = self.pos_buffer {
            gl.bind_buffer(GL::ARRAY_BUFFER, Some(pos_buffer));
            if let Some(ref sp) = self.shader_program {
                let (offset,stride) = self.offset_stride.get("position").unwrap_or(&(0,0));

                let att = gl.get_attrib_location(sp, "a_position");
                if att >= 0 {
                    gl.vertex_attrib_pointer_with_i32(att as u32, 3, GL::FLOAT, false, *stride, *offset);
                    gl.enable_vertex_attrib_array(att as u32);
                }
            }
        } else {
            gloo_console::log!("No position buffer found for model: ", self.name.clone());
            return;
        }

        // Normals
        if let Some(ref normal_buffer) = self.normal_buffer {
            gl.bind_buffer(GL::ARRAY_BUFFER, Some(normal_buffer));
            // set up a_normal attrib pointer
            if let Some(ref sp) = self.shader_program {
                let att = gl.get_attrib_location(sp, "a_normal");
                if att >= 0 {
                    let (offset,stride) = self.offset_stride.get("normal").unwrap_or(&(0,0));
                    
                    gl.vertex_attrib_pointer_with_i32(att as u32, 3, GL::FLOAT, false, *stride, *offset);
                    gl.enable_vertex_attrib_array(att as u32);
                }
            }
        }else {
            gloo_console::log!("No normal buffer found for model: ", self.name.clone());
            return;
        }

        // TexCoord
        if let Some(ref texcoord_buffer) = self.texcoord_buffer {
            gl.bind_buffer(GL::ARRAY_BUFFER, Some(texcoord_buffer));
            // set up a_normal attrib pointer
            if let Some(ref sp) = self.shader_program {
                let att = gl.get_attrib_location(sp, "a_texcoord");
                if att >= 0 {
                    let (offset,stride) = self.offset_stride.get("texcoord").unwrap_or(&(0,0));
                    gl.vertex_attrib_pointer_with_i32(att as u32, 2, GL::FLOAT, false, *stride, *offset);
                    gl.enable_vertex_attrib_array(att as u32);
                }
            }
        }else {
            gloo_console::log!("No normal buffer found for model: ", self.name.clone());
            return;
        }

        // Color                     
        if let Some(ref color_buffer) = self.color_buffer {
            gl.bind_buffer(GL::ARRAY_BUFFER, Some(color_buffer));
            if let Some(ref sp) = self.shader_program {
                let att = gl.get_attrib_location(sp, "a_color");
                if att >= 0 {
                    let (offset,stride) = self.offset_stride.get("color").unwrap_or(&(0,0));
                    gl.vertex_attrib_pointer_with_i32(att as u32, 3, GL::FLOAT, false, *stride, *offset);
                    gl.enable_vertex_attrib_array(att as u32);
                }
            }
        }else {
            gloo_console::log!("No color buffer found for model: ", self.name.clone());
            return;
        }

        if let Some(ref texture) = self.texture_buffer {
            gl.active_texture(GL::TEXTURE0);
            gl.bind_texture(GL::TEXTURE_2D, Some(texture));
            // If your shader expects a sampler uniform:
            if let Some(ref sp) = self.shader_program {
                let tex_loc = gl.get_uniform_location(sp, "u_texture");
                gl.uniform1i(tex_loc.as_ref(), 0); // 0 = GL_TEXTURE0
            }
        }

        if let Some(color) = &self.color {
            let color_loc = gl.get_uniform_location(&self.shader_program.as_mut().unwrap(), "u_color");
            gl.uniform3f(color_loc.as_ref(), color[0], color[1], color[2]);
        }   

        let scale_loc = gl.get_uniform_location(&self.shader_program.as_mut().unwrap(), "u_scale");
        gl.uniform3f(
            scale_loc.as_ref(),
            self.scale.x,
            self.scale.y,
            self.scale.z,
        );
        
        // let model_loc = gl.get_uniform_location(&self.shader_program.as_mut().unwrap(), "u_position");
        // let vals: [f32; 16] = self.matrix.to_array();
        // gl.uniform_matrix4fv_with_f32_array(model_loc.as_ref() , false, &vals);

        if self.use_transparency {
            gl.enable(GL::BLEND);
            gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        } else {
            gl.disable(GL::BLEND);
        }
        
        // Set the time uniform for the shader
        gl.uniform1f(self.time_location.as_ref() , time as f32);
        gl.draw_arrays(GL::TRIANGLES, 0, self.poly_count as i32);
    }

}