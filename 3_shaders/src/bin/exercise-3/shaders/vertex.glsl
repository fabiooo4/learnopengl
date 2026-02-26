#version 330 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;

out vec4 vertex_position;

uniform float triangle_offset;

void main() {
  vec4 new_pos = vec4(pos.x + triangle_offset, pos.yz, 1.0);
  vertex_position = new_pos;
  gl_Position = new_pos;
}
