use base64::prelude::*;
use gltf_json::{Accessor, accessor::{self, ComponentType}, mesh::Semantic, validation::Checked};
use gltf_json::mesh::*;

use gloo_console::log;

#[derive(Clone, Copy)]
pub struct _LocationRotation2D {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

impl _LocationRotation2D {
    pub fn _new(x: f32, y: f32, rotation: f32) -> Self {
        Self { x, y, rotation }
    }
}

pub struct _Location3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl _Location3D {
    pub fn _new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn _default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

pub fn _get_hex_from_rgb(col_str: String) -> String {
    let parts: Vec<&str> = col_str.split(',').collect();
    if parts.len() != 3 {
        log!("Invalid color format:", col_str);       
        return "#000000".to_string(); // Default to black if format is incorrect
    }
    let r = parts[0].trim().parse::<u8>().unwrap_or(0);
    let g = parts[1].trim().parse::<u8>().unwrap_or(0);
    let b = parts[2].trim().parse::<u8>().unwrap_or(0);
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub fn _get_rgb_from_hex(hex: &str) -> String {
    if hex.len() != 7 || !hex.starts_with('#') {
        log!("Invalid hex color format:", hex);
        return "0,0,0".to_string(); // Default to black if format is incorrect
    }
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
    format!("{},{},{}", r, g, b)
}
pub fn get_accessor_details(accessor: &Accessor) -> (i32, i32, u64) {
    // Calculate the number of bytes per component and the number of components
    let ty = match accessor.type_ {
        Checked::Valid(val) => val,
        Checked::Invalid => {
            log!("Invalid accessor type");   
            return (0, 0, 0);
        }                            
    };
    let component_count = get_component_count_for_accessor_type(ty);

    let comp_ty = match accessor.component_type {
        Checked::Valid(val) => {
            val
        },
        Checked::Invalid => {
            log!("Invalid component type");
            return (0, 0, 0);
        }                            
    };

    let byte_size = get_byte_size_for_component_type(comp_ty.0);
    let triangle_count =  accessor.count.0 as u64;

    (component_count, byte_size, triangle_count) 
}

fn get_component_count_for_accessor_type(ty: accessor::Type) -> i32 {
    match ty {
        accessor::Type::Scalar => 1,
        accessor::Type::Vec2 => 2,
        accessor::Type::Vec3 => 3,
        accessor::Type::Vec4 => 4,
        accessor::Type::Mat2 => 4,
        accessor::Type::Mat3 => 9,
        accessor::Type::Mat4 => 16
    }
}

fn get_byte_size_for_component_type(comp_ty: ComponentType) -> i32 {
    match comp_ty {
        ComponentType::I8 => 1,
        ComponentType::U8 => 1,
        ComponentType::I16 => 2,
        ComponentType::U16 => 2,
        ComponentType::U32 => 4,
        ComponentType::F32 => 4
    }
}

pub struct BufferObject {
    pub triangle_count: u64,
    pub buffer: Vec<u8>,
    pub _byte_stride: u64,
    pub _byte_offset: u64,
}

impl BufferObject {
    pub fn new(byte_stride: u64, byte_offset: u64, triangle_count: u64) -> Self {
        BufferObject {
            triangle_count: triangle_count,
            buffer: Vec::<u8>::new(),
            _byte_stride: byte_stride,
            _byte_offset: byte_offset,
        }
    }
}

pub fn get_data_from_buffer(primitive: &Primitive, gltf: &gltf_json::Root, semantic_in: Semantic, debug: bool) -> Result<BufferObject, String>{

    let semantic: Checked<Semantic> = Checked::<Semantic>::Valid(semantic_in.clone());
    let att = primitive.attributes.get(&semantic).unwrap();
    let accessor = gltf.accessors.get(att.value()).unwrap();
    let buffer_view = gltf.buffer_views.get(accessor.buffer_view.unwrap().value()).unwrap();
    let buffer = gltf.buffers.get(buffer_view.buffer.value()).unwrap();
    let (comp_count, comp_size, triangle_count) = get_accessor_details(accessor);
    if debug {
        gloo_console::log!(format!("BufferView details - byte_lenfth: {} , byte_offset {:?}, stride_length {}", buffer_view.byte_length.0, buffer_view.byte_offset.unwrap_or(gltf_json::validation::USize64::from(0 as usize)).0, buffer_view.byte_stride.unwrap().0));
        gloo_console::log!(format!("Getting data for semantic: {:?}, comp_count: {}, comp_size: {}, triangle_count: {}", semantic_in, comp_count, comp_size, triangle_count));
    }

    // let byte_stride = buffer_view.byte_stride.unwrap_or(Stride::default()).0 as u64;
    // let byte_offset = accessor.byte_offset.unwrap_or(gltf_json::validation::USize64::from(0 as usize)).0 as u64;
    let chunk_size = (comp_count * comp_size) as usize;
    let mut ret = BufferObject::new(chunk_size as u64, 0, triangle_count);
    // Check if the buffer data is embedded (base64) or external
    match &buffer.uri {
        Some(uri) => {
            if uri.starts_with("data:application/octet-stream;base64,") {
                // Handle embedded base64 data
                let b64_data = uri.split(',').nth(1).unwrap().as_bytes();
                let decoded = BASE64_STANDARD.decode(b64_data).unwrap();
                
                ret.buffer = Vec::<u8>::new();
                for i in 0..accessor.count.0 as usize {
                    let offset = (i * buffer_view.byte_stride.unwrap().0) + accessor.byte_offset.unwrap().0 as usize;
                    for j in 0..chunk_size {
                        ret.buffer.push(decoded[offset + j]);
                    }
                }
                Ok(ret)

            } else {
                // Handle external file URI
                // You'll need to load this file separately using your asset loading system
                let buffer_path = format!("assets/{}", uri);
                log!("Buffer path: ", buffer_path.clone());
                // Load buffer_path...
                Err(buffer_path)
            }
        },
        None => {
            log!("No buffer URI found");
            // Handle GLB-stored buffer data
            // For GLB files, you need to extract the buffer data from the binary chunk
            Err("".to_string())
        }
    }
}



pub struct _MinMax {
    pub min: _Location3D,
    pub max: _Location3D,
}

pub fn _get_min_max_for_buffer(primitive: &Primitive, gltf: &gltf_json::Root, semantic_in: Semantic, _debug: bool) -> Option<_MinMax> {
    let semantic: Checked<Semantic> = Checked::<Semantic>::Valid(semantic_in.clone());
    let att = primitive.attributes.get(&semantic).unwrap();
    let accessor = gltf.accessors.get(att.value()).unwrap();
    let min: _Location3D = match &accessor.min {
        Some(val) => {
            println!("Min values for accessor: {:?}", val);
            // _Location3D::_new(val[0] as f32, val[1] as f32, val[2] as f32)
            _Location3D::_default()
        },
        _ => return None
    };
    Some(_MinMax {
        min,
        max: _Location3D::_default() // Placeholder, as max is not handled in this example
    })
}

pub fn get_f32_buffer_from_u8(buffer: Vec<u8>) -> Vec<f32> {
    let mut ret = Vec::<f32>::new();
    for i in (0..buffer.len()).step_by(4) {
        // Convert each block of 4 to an f32        
        let bytes: [u8; 4] = [buffer[i], buffer[i+1], buffer[i+2], buffer[i+3]];
        let f = f32::from_le_bytes(bytes);
        ret.push(f);
    }

    ret
}

pub fn _distance_between_points(pt1: _LocationRotation2D, pt2: _LocationRotation2D) -> f32 {
    ((pt2.x - pt1.x).powi(2) + (pt2.y - pt1.y).powi(2)).sqrt()
}

pub fn _get_angle_between_points(pt1: _LocationRotation2D, pt2: _LocationRotation2D) -> f32 {
    (pt2.y - pt1.y).atan2(pt2.x - pt1.x) // Angle in radians
}

pub fn _get_angle_and_distance_between_points(pt1: _LocationRotation2D, pt2: _LocationRotation2D) -> (f32, f32) {
    (_get_angle_between_points(pt1, pt2), _distance_between_points(pt1, pt2))
}

pub fn _get_normalized_color(r: u8, g: u8, b: u8) -> [f32; 3] {
    [
        (r as f32) / 255.0,
        (g as f32) / 255.0,
        (b as f32) / 255.0
    ]
}

pub fn _get_angle_normalized(angle: f32) -> f32 {
    let mut ang = angle % (2.0 * std::f32::consts::PI);
    if ang < 0.0 {
        ang += 2.0 * std::f32::consts::PI;
    }
    ang
}