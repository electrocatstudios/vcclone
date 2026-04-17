precision mediump float;

attribute vec3 a_position;
attribute vec3 a_normal;
attribute vec2 a_texcoord;
attribute vec3 a_color;

// uniform sampler2D u_texture;
uniform float u_time;
uniform vec2 u_screensize;
uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_model;
uniform vec3 u_scale;
uniform vec3 u_rotation;
uniform vec3 u_color;
uniform vec3 u_color_override;

varying vec3 v_normal;
varying vec2 v_texcoord;
varying vec3 v_color;
varying float v_time;
varying vec2 v_screensize;
varying vec3 v_color_original;


mat4 rotationMatrix(vec3 rotation) {
    float cx = cos(rotation.x);
    float sx = sin(rotation.x);
    float cy = cos(rotation.y);
    float sy = sin(rotation.y);
    float cz = cos(rotation.z);
    float sz = sin(rotation.z);

    mat4 rotX = mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, cx, -sx, 0.0,
        0.0, sx, cx, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    mat4 rotY = mat4(
        cy, 0.0, sy, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sy, 0.0, cy, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    mat4 rotZ = mat4(
        cz, -sz, 0.0, 0.0,
        sz, cz, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    return rotZ * rotY * rotX; // Combine rotations in ZYX order
}

void main() {
    // Pass through varyings
    v_normal = (u_model * vec4(a_normal, 0.0)).xyz;
    v_texcoord = a_texcoord;
    v_color = u_color;
    v_time = u_time;
    v_screensize = u_screensize;
    v_color_original = a_color;
    
    // Adjust the position based on the screen size
    vec4 scaledPosition = vec4(a_position * u_scale, 1.0);

    // Apply rotation
    mat4 rotationMat = rotationMatrix(u_rotation);
    vec4 rotatedPosition = rotationMat * scaledPosition;

    // Transform to world space
    vec4 worldPosition = u_model * rotatedPosition;

    // Transform to view and projection space
    vec4 viewPosition = u_view * worldPosition;
    vec4 projectedPosition = u_projection * viewPosition;
    
    // Apply screen size correction
    vec2 uv = projectedPosition.xy; 
    gl_Position = vec4(uv, projectedPosition.z, projectedPosition.w);
}
