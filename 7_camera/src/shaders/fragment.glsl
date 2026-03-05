#version 330 core

uniform sampler2D foreground_texture;
uniform sampler2D background_texture;

in vec4 vertex_color;
in vec2 texture_coords;
out vec4 frag_color;

void main() {
  vec4 bg = texture(background_texture, texture_coords);
  vec4 fg = texture(foreground_texture, texture_coords);
  frag_color = vec4(mix(bg.rgb, fg.rgb, fg.a), 1.0);
}
