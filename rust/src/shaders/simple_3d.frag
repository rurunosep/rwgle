precision mediump float;

varying vec3 v_normal;

uniform vec3 u_reverse_light_direction;

void main() {
  vec3 normal = normalize(v_normal);
  float light = max(dot(normal, u_reverse_light_direction), 0.1);
  vec3 color = vec3(153, 255, 51) / 255.0 * light;

  gl_FragColor = vec4(color, 1);
}