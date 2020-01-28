#version 300 es
precision mediump float;
precision mediump sampler2DArray;
out vec4 color;

in vec3 vColor;
in vec2 vTex;
// flat in int texture_index;

uniform sampler2DArray tex;

void main() {
  /// no clue why this doesn't seem to do anything.
  color = texture(tex, vec3(vTex, 2));
}
