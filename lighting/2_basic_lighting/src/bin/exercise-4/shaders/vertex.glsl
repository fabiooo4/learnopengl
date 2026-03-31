#version 330 core
layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 norm;
layout(location = 2) in vec2 texture;
layout(location = 3) in vec3 color;

// Projection
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

// Lighting
uniform vec3 light_color;
uniform vec3 light_pos;
uniform vec3 camera_pos;

out vec3 vertex_color;

void main() {
  gl_Position = projection * view * model * vec4(pos, 1.0);

  vec3 world_vertex_pos = vec3(model * vec4(pos, 1.0));
  vec3 normal = mat3(transpose(inverse(model))) * norm; // Costly operation

  // Ambient light
  float ambient_strength = 0.1;
  vec3 ambient_light = light_color * ambient_strength;

  // Diffuse light
  vec3 vertex_normal = normalize(normal);
  vec3 light_direction = normalize(light_pos - world_vertex_pos);
  float diffuse = max(dot(vertex_normal, light_direction), 0.0);
  vec3 diffuse_light = diffuse * light_color;

  // Specular light
  float specular_strength = 0.5;
  vec3 view_direction = normalize(camera_pos - world_vertex_pos);
  // The light direction needs to point from the light to the fragment, so it is negated
  vec3 reflected_direction = reflect(-light_direction, vertex_normal);
  int shine = 32;
  float specular = pow(max(dot(view_direction, reflected_direction), 0.0), shine);
  vec3 specular_light = specular_strength * specular * light_color;  

  vec3 lighting = (ambient_light + diffuse_light + specular_light) * color;

  vertex_color = lighting;
}
