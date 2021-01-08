attribute vec3 a_position;
attribute vec3 a_normal;

uniform mat4 u_world;
uniform mat4 u_view;
uniform mat4 u_projection;
uniform mat4 u_world_inverse_transpose;
uniform vec3 u_camera_world_position;

varying vec3 v_normal;
varying vec3 v_surface_to_camera;

void main() {
  vec3 world_position = (u_world * vec4(a_position, 1)).xyz;

  v_surface_to_camera = u_camera_world_position - world_position;

  gl_Position = u_projection * u_view * vec4(world_position, 1);

  v_normal = mat3(u_world_inverse_transpose) * a_normal;
}