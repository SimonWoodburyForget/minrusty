#version 300 es
precision mediump float;
precision mediump sampler2DArray;
out vec4 color;

in vec3 vColor;
in vec2 vTex;

uniform sampler2DArray tex;

void main() {
  color = texture(tex, vec3(vTex, 1)) // * vec4(vColor, 1.0)
    ;
}
