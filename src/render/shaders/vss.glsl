#version 300 es
in vec3 aPos;
in vec3 aColor;
in vec2 aTex;

out vec3 vColor;
out vec2 vTex;

void main() {
  gl_Position = vec4(aPos, 1.0);
  vColor = aColor;
  vTex = aTex;
}
