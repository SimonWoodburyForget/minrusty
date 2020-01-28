#version 300 es
precision mediump float;
precision mediump sampler2DArray;
in vec3 vColor;
in vec2 vTex;

out vec4 color;

uniform sampler2DArray tex;

void main() {
  color = texture(tex, vec3(vTex, 0));
}
