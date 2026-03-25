#version 330 core

uniform vec3 light_color;
uniform vec3 light_pos;
uniform vec3 camera_pos;

in vec3 vertex_color;
in vec3 fragment_pos;
in vec3 normal;

out vec4 frag_color;

void main() {

  // Ambient light
  float ambient_strength = 0.08;
  vec3 ambient_light = light_color * ambient_strength;

  // Diffuse light
  float diffuse_strength = 1.5;
  vec3 fragment_normal = normalize(normal);
  vec3 light_direction = normalize(light_pos - fragment_pos);
  float diffuse = max(dot(fragment_normal, light_direction), 0.0);
  vec3 diffuse_light = diffuse_strength * diffuse * light_color;

  // Specular light
  float specular_strength = 5.5;
  vec3 view_direction = normalize(camera_pos - fragment_pos);
  // The light direction needs to point from the light to the fragment, so it is negated
  vec3 reflected_direction = reflect(-light_direction, fragment_normal);
  float shine = pow(2, 8);
  float specular = pow(max(dot(view_direction, reflected_direction), 0.0), shine);
  vec3 specular_light = specular_strength * specular * light_color;  

  vec3 lighting = (ambient_light + diffuse_light + specular_light) * vertex_color;

  frag_color = vec4(lighting, 1.0);
}
