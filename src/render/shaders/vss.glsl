#version 300 es
in vec3 aPos;
in vec3 aColor;
in vec2 aTex;
// in int aTextureIndex;

out vec3 vColor;
out vec2 vTex;
// flat out int texture_index;

uniform mat4 transform;

void main() {
  gl_Position = transform * vec4(aPos, 1.0);
  vColor = aColor;
  vTex = aTex;
  // texture_index = aTextureIndex;
  // ?? = gl_InstanceID;
}
