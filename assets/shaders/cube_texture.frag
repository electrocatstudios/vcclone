precision mediump float;

uniform sampler2D u_texture;

varying vec3 v_normal;
varying vec2 v_texcoord;

varying vec3 v_color;
varying vec3 v_color_original;
varying float v_time;

void main() {
    vec4 texture_color = texture2D(u_texture, v_texcoord);
    // if (texture_color.a < 0.5) {
    //     gl_FragColor = vec4(1.0, 0.0, 0.0, 0.0);
    // } else {
    gl_FragColor = texture_color;
    // }
    
}