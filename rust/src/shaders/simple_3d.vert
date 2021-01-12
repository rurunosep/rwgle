// I'm doing this just so that precision of uniforms matches between
// vert and frag shaders without making them explicit
precision mediump float;

attribute vec3 a_position;
attribute vec2 a_texcoords;
attribute vec3 a_normal;
attribute vec3 a_tangent;
attribute vec3 a_bitangent;

uniform mat4 u_world;
uniform mat4 u_view;
uniform mat4 u_projection;
uniform mat4 u_world_inverse_transpose;

varying vec3 v_normal;
varying vec3 v_position;
varying vec2 v_texcoords;
varying vec3 v_tangent;
varying vec3 v_bitangent;

void main() {
  vec3 world_position = (u_world * vec4(a_position, 1)).xyz;

  v_position = world_position;
  v_normal = mat3(u_world_inverse_transpose) * a_normal;
  v_texcoords = a_texcoords;
  v_tangent = a_tangent;
  v_bitangent = a_bitangent;

  gl_Position = u_projection * u_view * vec4(world_position, 1);
}