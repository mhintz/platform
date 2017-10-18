// #version 410
//
// in vec3 a_position;
// in vec4 a_color;
//
// uniform mat4 uProjectionMatrix;
// uniform mat4 uViewMatrix;
// uniform mat4 uModelMatrix;
// uniform mat4 uNormalMatrix;
//
// out vec4 color;
//
// void main() {
//   color = a_color;
//   gl_Position = vec4(a_position, 1.0);
// }

#version 150 core

in vec3 a_position;
in vec4 a_color;
// in vec3 a_normal;
// in vec2 a_tex_coord0;

out vec4 v_Color;

void main() {
    v_Color = vec4(a_color.xyz, 1.0);
    gl_Position = vec4(a_position.xy, 0.0, 1.0);
}
