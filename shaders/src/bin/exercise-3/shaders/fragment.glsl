#version 330 core

uniform vec4 sum_color;

out vec4 frag_color;
in vec4 vertex_position;

void main() {
  frag_color = vertex_position;
}
