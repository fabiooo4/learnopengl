#version 330 core

uniform vec4 sum_color;

out vec4 frag_color;
in vec4 vertex_color;


void main() {
  frag_color = vertex_color + sum_color;
}
