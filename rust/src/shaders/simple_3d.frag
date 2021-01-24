#define MAX_LIGHTS 5

precision mediump float;

struct Light {
  vec3 position;
  vec3 color;
  float attentuation_coefficient;
  bool directional; // Maybe store this in the W?
};

varying vec3 v_normal;  // Surface normal in world space
varying vec3 v_position;  // In world space
varying vec2 v_texcoords;
varying vec3 v_tangent;
varying vec3 v_bitangent;

uniform mat4 u_world;
uniform vec3 u_camera_position;
uniform Light u_lights[MAX_LIGHTS];
uniform lowp int u_num_lights;

uniform sampler2D u_color_map;
uniform sampler2D u_specular_map;
uniform sampler2D u_normal_map;

// TODO: Organize all of this

void main() {
  float ambient_coefficient = 0.1;
  float specular_exponent = 70.0; // Can I get this from the specular map?

  vec4 material_color = texture2D(u_color_map, v_texcoords);
  float smoothness = texture2D(u_specular_map, v_texcoords).r;
  vec3 normal = normalize(texture2D(u_normal_map, v_texcoords).rgb * 2.0 - 1.0);

  vec3 surface_normal = normalize(v_normal);
  vec3 tangent = normalize((u_world * vec4(v_tangent, 0.0)).xyz);
  vec3 bitangent = normalize((u_world * vec4(v_bitangent, 0.0)).xyz);
  mat3 to_tangent_space = mat3(
    tangent.x, bitangent.x, surface_normal.x,
    tangent.y, bitangent.y, surface_normal.y,
    tangent.z, bitangent.z, surface_normal.z
  );

  vec3 surface_to_camera = to_tangent_space * (u_camera_position - v_position);
  vec3 surface_to_camera_dir = normalize(surface_to_camera);

  vec3 diffuse_sum = vec3(0);
  vec3 specular_sum = vec3(0);

  for (int i = 0; i < MAX_LIGHTS; i++) {
    if (i == u_num_lights) break;

    vec3 surface_to_light = to_tangent_space * (u_lights[i].position - v_position);

    float attentuation = 1.0 / (1.0 +
      pow(u_lights[i].attentuation_coefficient * length(surface_to_light), 2.0));

    // If the light is directional, it's "position" is its inverse direction
    vec3 surface_to_light_dir = normalize(
        u_lights[i].directional ? u_lights[i].position : surface_to_light
      );

    // Diffuse
    float diffuse_coefficient = attentuation *
      max(0.0, dot(normal, surface_to_light_dir));
    diffuse_sum += diffuse_coefficient * u_lights[i].color;
    
    // Specular
    vec3 half_vector = normalize(surface_to_light_dir + surface_to_camera_dir);
    float dot_half_normal = max(0.0, dot(normal, half_vector));
    float specular_coefficient = attentuation * pow(dot_half_normal, specular_exponent);
    specular_sum += specular_coefficient * u_lights[i].color;
  }

  vec3 ambient_component = material_color.rgb * ambient_coefficient;
  vec3 diffuse_component = material_color.rgb *
    (diffuse_sum / max(diffuse_sum.r, max(diffuse_sum.g, max(diffuse_sum.b, 1.0))));
  vec3 specular_component = smoothness *
    (specular_sum / max(specular_sum.r, max(specular_sum.g, max(specular_sum.b, 1.0))));

  gl_FragColor = vec4((ambient_component + diffuse_component + specular_component), 1);
}