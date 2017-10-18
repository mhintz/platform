// #version 410
//
// in vec4 color;
//
// out vec4 Target0;
//
// void main() {
//   Target0 = color;
// }

#version 150 core

in vec4 v_Color;
out vec4 Target0;

void main() {
    Target0 = v_Color;
}
