#version 300 es
in vec3 aPos;
in vec3 aColor; 

out vec3 vertexColor; 

void main() {
  gl_Position = vec4(aPos, 1.0);
  vertexColor = aColor;
}
