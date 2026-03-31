#version 330 core
layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 norm;
layout(location = 2) in vec2 texture;
layout(location = 3) in vec3 color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 world_light_pos;

out vec3 vertex_color;
out vec3 fragment_pos;
out vec3 normal;
out vec3 light_pos;

void main() {
  gl_Position = projection * view * model * vec4(pos, 1.0);

  fragment_pos = vec3(view * model * vec4(pos, 1.0));
  normal = mat3(transpose(inverse(view * model))) * norm; // Costly operation
  light_pos = vec3(view * vec4(world_light_pos, 1.0));
  vertex_color = color;
}
