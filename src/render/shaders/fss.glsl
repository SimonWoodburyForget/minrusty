#version 300 es
precision mediump float;
precision mediump sampler2DArray;

in vec2 f_text_pos;
flat in int id;

out vec4 color;

uniform sampler2DArray tex;

void main() {
  color = texture(tex, vec3(f_text_pos, id));
}
