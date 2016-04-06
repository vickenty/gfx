#version 150

in vec4 vertex_pos;

uniform mat4 transform;

out vec4 color;

void main() {
  gl_Position = transform * vertex_pos;
  color = vec4(0.4, 0.3, 0.2, 1.0);
}
