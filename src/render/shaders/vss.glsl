#version 300 es
in vec3 aPos;
in vec3 aColor;
in vec2 aTex;
in vec2 aTile;

out vec3 vColor;
out vec2 vTex;
flat out int id;

uniform mat4 transform;

void main() {
  mat4 tiling = mat4(1.0, 0.0, 0.0, 0.0,
                     0.0, 1.0, 0.0, 0.0,
                     0.0, 0.0, 1.0, 0.0,
                     aTile.x, aTile.y, 0.0, 1.0);
  vec4 pos = vec4(aPos, 1.0);
  gl_Position = transform * tiling * pos;
  vColor = aColor;
  vTex = aTex;
  id = gl_InstanceID;
}
