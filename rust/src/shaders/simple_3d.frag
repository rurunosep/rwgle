#define MAX_LIGHTS 5

precision mediump float;

struct Light {
  vec3 position;
  vec3 color;
  float attentuation_coefficient;
  bool directional;
};

varying vec3 v_normal;
varying vec3 v_world_position;
varying vec2 v_texcoords;

uniform vec3 u_camera_position;
uniform Light u_lights[MAX_LIGHTS];
uniform lowp int u_num_lights;
uniform sampler2D u_texture;

// TODO: Organize all of this

void main() {
  vec3 normal = normalize(v_normal);
  vec3 surface_to_camera_dir = normalize(u_camera_position - v_world_position);

  // TODO: Make these uniforms
  // vec3 material_color = vec3(153, 0, 0) / 255.0;
  vec3 material_specular_color = vec3(1);
  float ambient_coefficient = 0.1;
  float specular_exponent = 70.0;

  vec3 diffuse_sum = vec3(0);
  vec3 specular_sum = vec3(0);

  for (int i = 0; i < MAX_LIGHTS; i++) {
    if (i == u_num_lights) break;

    vec3 surface_to_light = u_lights[i].position - v_world_position;
    float attentuation = 1.0 / (1.0 +
      pow(u_lights[i].attentuation_coefficient * length(surface_to_light), 2.0));
    // If the light is directional, it's "position" is its inverse direction
    vec3 surface_to_light_dir = normalize(
        u_lights[i].directional ? u_lights[i].position : surface_to_light
      );

    float diffuse_coefficient = attentuation *
      max(0.0, dot(normal, surface_to_light_dir));
    diffuse_sum += diffuse_coefficient * u_lights[i].color;
    
    // Specular
    vec3 half_vector = normalize(surface_to_light_dir + surface_to_camera_dir);
    float dot_half_normal = max(0.0, dot(normal, half_vector));
    float specular_coefficient = attentuation * pow(dot_half_normal, specular_exponent);
    specular_sum += specular_coefficient * u_lights[i].color;
  }

  vec3 ambient_component = texture2D(u_texture, v_texcoords).rgb * ambient_coefficient;
  vec3 diffuse_component = texture2D(u_texture, v_texcoords).rgb *
    (diffuse_sum / max(diffuse_sum.r, max(diffuse_sum.g, max(diffuse_sum.b, 1.0))));
  vec3 specular_component = material_specular_color *
    (specular_sum / max(specular_sum.r, max(specular_sum.g, max(specular_sum.b, 1.0))));

  gl_FragColor = vec4((ambient_component + diffuse_component + specular_component), 1);
}