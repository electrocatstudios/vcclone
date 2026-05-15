precision mediump float;

uniform sampler2D u_texture;

uniform vec3 u_color;

varying vec3 v_normal;
varying vec2 v_texcoord;

varying vec3 v_color;
varying vec3 v_color_original;
varying float v_time;

void main() {
    // gl_FragColor = vec4(col_out, v_texcoord.x);
    gl_FragColor = vec4(u_color, 1.0);
}