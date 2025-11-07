#version 330 core
// Input vertex attributes
layout(location = 0) in vec3 position;

void main()
{
  // gl_Position is a built-in output variable that holds the final position of the vertex
  gl_Position = vec4(position, 1.0);
}
