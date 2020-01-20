#version 300 es
precision mediump float;
out vec4 color;
in vec4 vertexColor;
uniform vec4 ourColor;

void main() {
  color = ourColor;
}
