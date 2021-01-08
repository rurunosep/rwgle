precision mediump float;

varying vec3 v_normal;
varying vec3 v_surface_to_camera;

uniform vec3 u_reverse_light_direction;

void main() {
  vec3 normal = normalize(v_normal);
  vec3 surface_to_camera = normalize(v_surface_to_camera);

  vec3 material_color = vec3(153, 255, 51) / 255.0;

  // Ambient
  float ambient_coefficient = 0.1; // TODO: uniform
  vec3 ambient_component = material_color * ambient_coefficient;

  // Diffuse
  float diffuse_coefficient = max(0.0, dot(normal, u_reverse_light_direction));
  vec3 diffuse_component = material_color * diffuse_coefficient;

  // Specular
  // TODO: rename shit and clean up calculations
  vec3 half_vector = normalize(u_reverse_light_direction + surface_to_camera);
  float dot_half_normal = max(0.0, dot(normal, half_vector));
  float specular_exponent = 10.0; // TODO: uniform
  float specular_coefficient = pow(dot_half_normal, specular_exponent);
  vec3 specular_component = vec3(1.0, 1.0, 1.0) * specular_coefficient;

  gl_FragColor = vec4((ambient_component + diffuse_component + specular_component), 1);
}