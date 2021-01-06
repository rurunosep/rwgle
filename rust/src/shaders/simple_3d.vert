attribute vec3 a_position;
attribute vec3 a_normal;

uniform mat4 u_world_inverse_transpose;
uniform mat4 u_world_view_projection;

varying vec3 v_normal;

void main() {
  v_normal = mat3(u_world_inverse_transpose) * a_normal;

  gl_Position = u_world_view_projection * vec4(a_position, 1);
}