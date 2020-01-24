#version 300 es
precision mediump float;
out vec4 color;

in vec3 vColor;
in vec2 vTex;

uniform sampler2D tex;

void main() {
  // FIXME:
  // - texture(tex, vTex) doesn't work on WebGL for some reason
  color = texture(tex, vTex) * vec4(vColor, 1.0);
}
