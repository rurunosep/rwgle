attribute vec4 position;
attribute vec3 color;

uniform mat4 transform;

varying vec3 v_color;

void main() {
  v_color = color;
  gl_Position = transform * position;
}