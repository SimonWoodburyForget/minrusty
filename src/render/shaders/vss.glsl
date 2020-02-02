#version 300 es
in vec2 vertex_pos;
in vec2 text_pos;
in vec2 tile_pos;

out vec2 f_text_pos;
flat out int id;

uniform mat4 transform;

void main() {
  f_text_pos = text_pos;
  
  vec4 pos = vec4(vertex_pos, 1.0, 1.0);
  mat4 tile = mat4(vec4(1.0, 0.0, 0.0, 0.0),
                   vec4(0.0, 1.0, 0.0, 0.0),
                   vec4(0.0, 0.0, 1.0, 0.0),
                   vec4(tile_pos, 0.0, 1.0));
  
  gl_Position = transform * tile * pos;
  id = gl_InstanceID;
}
