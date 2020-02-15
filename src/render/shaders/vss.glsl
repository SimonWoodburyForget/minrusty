#version 300 es
in vec2 vert_pos;
in vec2 text_pos;
in uint text_idx;

out vec2 f_text_pos;
flat out uint idx;

uniform mat4 transform;

void main() {
  f_text_pos = text_pos;
  idx = text_idx;
  
  vec4 pos = vec4(vert_pos, 1.0, 1.0);
  
  // mat4 tile = mat4(vec4(1.0, 0.0, 0.0, 0.0),
  //                  vec4(0.0, 1.0, 0.0, 0.0),
  //                  vec4(0.0, 0.0, 1.0, 0.0),
  //                  vec4(tile_pos.xy, tile_pos.z*0.01, 1.0));

  // mat4 size = mat4(vec4(tile_size, 0.0, 0.0, 0.0),
  //                  vec4(0.0, tile_size, 0.0, 0.0),
  //                  vec4(0.0, 0.0, 1.0, 0.0),
  //                  vec4(0.0, 0.0, 0.0, 1.0));

  // mat4 size_pos = mat4(vec4(1.0, 0.0, 0.0, 0.0),
  //                      vec4(0.0, 1.0, 0.0, 0.0),
  //                      vec4(0.0, 0.0, 1.0, 0.0),
  //                      vec4(tile_size/2.0,
  //                           tile_size/2.0,
  //                           0.0, 1.0));
  
  gl_Position = transform * pos;
}
