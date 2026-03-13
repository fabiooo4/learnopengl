#version 330 core
layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;
layout(location = 2) in vec2 texture;

out vec4 vertex_color;
out vec2 texture_coords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
  gl_Position = projection * view * model * vec4(pos, 1.0);
  vertex_color = vec4(color, 1.0);
  texture_coords = texture;
}
