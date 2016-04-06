#version 150

in vec4 vertex_pos;
in mat4 transform;

out vec4 out_pos;

void main() {
  out_pos = transform * vertex_pos;
}
