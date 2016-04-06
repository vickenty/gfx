#version 150

in vec4 color;
out vec4 out_color;

const vec4 FOG = vec4(0.1, 0.1, 0.1, 1.0);

void main() {
  out_color = mix(color, FOG, pow(gl_FragCoord.z, 2.0));
}
