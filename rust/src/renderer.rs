use super::mesh::*;
use super::model::*;
use super::object::*;
use super::utils::*;
use nalgebra_glm as na;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlShader, WebGlUniformLocation};

pub struct Renderer {
  program: WebGlProgram,
  uniform_locations: HashMap<String, WebGlUniformLocation>,
  redstone: Object,
  nether_gold: Object,
  //
  camera_direction_index: usize,
  camera_rotation: na::Quat,
  camera_transition: Option<CameraRotationTransition>,
  //
  redstone_l: Object,
  redstone_r: Object,
  redstone_b: Object,
}

impl Renderer {
  pub async fn new(gl: &GL) -> Result<Renderer, String> {
    let program = link_program(
      &gl,
      include_str!("./shaders/simple_3d.vert"),
      include_str!("./shaders/simple_3d.frag"),
    )?;

    let uniform_locations = get_uniform_locations(&gl, &program);

    let cube_mesh = Rc::new(Mesh::new(&gl, &get_attributes(&gl, &program), "cube.gltf").await);

    let redstone_model = Rc::new(Model {
      mesh: Rc::clone(&cube_mesh),
      color_map: load_texture(&gl, "redstone_block.png"),
      specular_map: load_texture(&gl, "redstone_block_s.png"),
      normal_map: load_texture(&gl, "redstone_block_n.png"),
    });
    let nether_gold_model = Rc::new(Model {
      mesh: Rc::clone(&cube_mesh),
      color_map: load_texture(&gl, "nether_gold_ore.png"),
      specular_map: load_texture(&gl, "nether_gold_ore_s.png"),
      normal_map: load_texture(&gl, "nether_gold_ore_n.png"),
    });

    let redstone = Object {
      model: Rc::clone(&redstone_model),
      scale: na::scaling(&na::vec3(100., 100., 100.)),
      rotation: na::identity(),
      translation: na::translation(&na::vec3(200., 0., -700.)),
    };

    let nether_gold = Object {
      model: Rc::clone(&nether_gold_model),
      scale: na::scaling(&na::vec3(100., 100., 100.)),
      rotation: na::identity(),
      translation: na::translation(&na::vec3(-200., 0., -700.)),
    };

    // --------------
    let mut redstone_l = redstone.clone();
    redstone_l.translation = na::translation(&na::vec3(-700., 0., 0.));

    let mut redstone_r = redstone.clone();
    redstone_r.translation = na::translation(&na::vec3(700., 0., 0.));

    let mut redstone_b = redstone.clone();
    redstone_b.translation = na::translation(&na::vec3(0., 0., 700.));
    // --------------

    Ok(Renderer {
      program,
      uniform_locations,
      redstone,
      nether_gold,
      //
      camera_direction_index: 0,
      camera_rotation: na::quat_inverse(&na::quat_look_at(
        &na::vec3(0., 0., -1.),
        &na::vec3(0., 1., 0.),
      )),
      camera_transition: None,
      //
      redstone_l,
      redstone_r,
      redstone_b,
    })
  }

  pub fn render(&mut self, gl: &GL) {
    let time = web_sys::window().unwrap().performance().unwrap().now() as f32;

    // Rotate camera smoothly
    if let Some(camera_transition) = &mut self.camera_transition {
      camera_transition.update(&mut self.camera_rotation);
      if camera_transition.finished {
        self.camera_transition = None;
      }
    }

    gl.use_program(Some(&self.program));
    self.load_uniforms(gl);

    // Redstone
    self.redstone.rotation = na::rotation(0., &na::vec3(1., 0., 0.))
      * na::rotation(
        std::f32::consts::PI * 2. * (time % 10000. / 10000.),
        &na::vec3(0., 1., 0.),
      )
      * na::rotation(std::f32::consts::PI * 0., &na::vec3(0., 0., 1.));

    self.redstone.render(&gl, &self.uniform_locations);

    // Nether gold
    self.nether_gold.rotation = na::rotation(0., &na::vec3(1., 0., 0.))
      * na::rotation(
        std::f32::consts::PI * 2. * -(time % 10000. / 10000.),
        &na::vec3(0., 1., 0.),
      )
      * na::rotation(std::f32::consts::PI * 0., &na::vec3(0., 0., 1.));

    self.nether_gold.render(&gl, &self.uniform_locations);

    // Other redstones
    self.redstone_l.rotation = na::rotation(0., &na::vec3(1., 0., 0.))
      * na::rotation(
        std::f32::consts::PI * 2. * (time % 10000. / 10000.),
        &na::vec3(0., 1., 0.),
      )
      * na::rotation(std::f32::consts::PI * 0., &na::vec3(0., 0., 1.));
    self.redstone_l.render(&gl, &self.uniform_locations);

    self.redstone_r.rotation = na::rotation(0., &na::vec3(1., 0., 0.))
      * na::rotation(
        std::f32::consts::PI * 2. * (time % 10000. / 10000.),
        &na::vec3(0., 1., 0.),
      )
      * na::rotation(std::f32::consts::PI * 0., &na::vec3(0., 0., 1.));
    self.redstone_r.render(&gl, &self.uniform_locations);

    self.redstone_b.rotation = na::rotation(0., &na::vec3(1., 0., 0.))
      * na::rotation(
        std::f32::consts::PI * 2. * (time % 10000. / 10000.),
        &na::vec3(0., 1., 0.),
      )
      * na::rotation(std::f32::consts::PI * 0., &na::vec3(0., 0., 1.));
    self.redstone_b.render(&gl, &self.uniform_locations);
  }

  pub fn rotate_camera_left(&mut self) {
    self.camera_direction_index = (self.camera_direction_index + 3) % 4;
    let new_camera_rotation = na::quat_inverse(&na::quat_look_at(
      &match self.camera_direction_index {
        0 => na::vec3(0., 0., -1.),
        1 => na::vec3(1., 0., 0.),
        2 => na::vec3(0., 0., 1.),
        _ => na::vec3(-1., 0., 0.),
      },
      &na::vec3(0., 1., 0.),
    ));

    let now = web_sys::window().unwrap().performance().unwrap().now() as f32;
    self.camera_transition = Some(CameraRotationTransition::new(
      self.camera_rotation,
      new_camera_rotation,
      now,
      now + 1000.,
    ));
  }

  pub fn rotate_camera_right(&mut self) {
    self.camera_direction_index = (self.camera_direction_index + 1) % 4;
    let new_camera_rotation = na::quat_inverse(&na::quat_look_at(
      &match self.camera_direction_index {
        0 => na::vec3(0., 0., -1.),
        1 => na::vec3(1., 0., 0.),
        2 => na::vec3(0., 0., 1.),
        _ => na::vec3(-1., 0., 0.),
      },
      &na::vec3(0., 1., 0.),
    ));

    let now = web_sys::window().unwrap().performance().unwrap().now() as f32;
    self.camera_transition = Some(CameraRotationTransition::new(
      self.camera_rotation,
      new_camera_rotation,
      now,
      now + 1000.,
    ));
  }

  fn load_uniforms(&self, gl: &GL) {
    // View
    let camera_rotation = na::quat_to_mat4(&self.camera_rotation);
    let camera_position = na::vec3(0., 0., 0.);
    let camera_translation = na::translation(&camera_position);
    let view = na::inverse(&(camera_translation * camera_rotation));
    gl.uniform_matrix4fv_with_f32_array(
      Some(&self.uniform_locations.get("u_view").unwrap()),
      false,
      &view.as_slice(),
    );

    // Projection
    let canvas = gl
      .canvas()
      .unwrap()
      .dyn_into::<HtmlCanvasElement>()
      .unwrap();
    let fov = 60. * (std::f32::consts::PI / 180.);
    let aspect = (canvas.width() / canvas.height()) as f32;
    let projection = na::perspective(aspect, fov, 1., 2000.);
    gl.uniform_matrix4fv_with_f32_array(
      Some(&self.uniform_locations.get("u_projection").unwrap()),
      false,
      &projection.as_slice(),
    );

    // Camera World Position
    gl.uniform3fv_with_f32_array(
      Some(&self.uniform_locations.get("u_camera_position").unwrap()),
      camera_position.as_slice(),
    );

    // Lights
    self.make_light(&gl, 0, &[0., 0., 0.], &[1., 1., 1.], 0.001);
    //self.make_light(&gl, 1, &[0., 0., -2000.], &[1., 1., 1.], 0.001);
    //self.make_light(&gl, 2, &[800., 0., -800.], &[1., 1., 1.], 0.001);
    gl.uniform1i(
      Some(&self.uniform_locations.get("u_num_lights").unwrap()),
      1,
    );

    // Textures
    gl.uniform1i(Some(&self.uniform_locations.get("u_color_map").unwrap()), 0);
    gl.uniform1i(
      Some(&self.uniform_locations.get("u_specular_map").unwrap()),
      1,
    );
    gl.uniform1i(
      Some(&self.uniform_locations.get("u_normal_map").unwrap()),
      2,
    );
  }

  fn make_light(&self, gl: &GL, id: i32, pos: &[f32; 3], color: &[f32; 3], attentuation: f32) {
    gl.uniform3fv_with_f32_array(
      Some(
        &self
          .uniform_locations
          .get(&format!("u_lights[{}].position", id))
          .unwrap(),
      ),
      pos,
    );
    gl.uniform3fv_with_f32_array(
      Some(
        &self
          .uniform_locations
          .get(&format!("u_lights[{}].color", id))
          .unwrap(),
      ),
      color,
    );
    gl.uniform1f(
      Some(
        &self
          .uniform_locations
          .get(&format!("u_lights[{}].attentuation_coefficient", id))
          .unwrap(),
      ),
      attentuation,
    );
  }
}

struct CameraRotationTransition {
  finished: bool,
  start_quat: na::Quat,
  end_quat: na::Quat,
  start_time: f32,
  end_time: f32,
}

impl CameraRotationTransition {
  pub fn new(
    start_quat: na::Quat,
    end_quat: na::Quat,
    start_time: f32,
    end_time: f32,
  ) -> CameraRotationTransition {
    CameraRotationTransition {
      finished: false,
      start_quat,
      end_quat,
      start_time,
      end_time,
    }
  }

  pub fn update(&mut self, camera_rotation: &mut na::Quat) {
    if !self.finished {
      let now = web_sys::window().unwrap().performance().unwrap().now() as f32;
      let x = (now - self.start_time) / (self.end_time - self.start_time);
      let y = ease_out(x);
      *camera_rotation = na::quat_slerp(&self.start_quat, &self.end_quat, y);
      if x >= 1. {
        self.finished = true;
      }
    }
  }
}

fn ease_out(x: f32) -> f32 {
  1. - (1. - x).powi(5)
}

fn link_program(
  gl: &GL,
  vertex_source: &str,
  fragment_source: &str,
) -> Result<WebGlProgram, String> {
  let program = gl.create_program().unwrap();

  let vertex_shader = compile_shader(&gl, GL::VERTEX_SHADER, vertex_source)?;
  let fragment_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, fragment_source)?;

  gl.attach_shader(&program, &vertex_shader);
  gl.attach_shader(&program, &fragment_shader);
  gl.link_program(&program);

  if gl
    .get_program_parameter(&program, GL::LINK_STATUS)
    .as_bool()
    .unwrap()
  {
    Ok(program)
  } else {
    Err(gl.get_program_info_log(&program).unwrap())
  }
}

fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
  let shader = gl.create_shader(shader_type).unwrap();
  gl.shader_source(&shader, &source);
  gl.compile_shader(&shader);

  if gl
    .get_shader_parameter(&shader, GL::COMPILE_STATUS)
    .as_bool()
    .unwrap()
  {
    Ok(shader)
  } else {
    Err(gl.get_shader_info_log(&shader).unwrap())
  }
}

pub struct Attribute {
  pub name: String,
  pub index: u32,
  pub size: i32,
  pub type_: u32,
}

fn get_attributes(gl: &GL, program: &WebGlProgram) -> HashMap<String, Attribute> {
  let num_attributes = gl
    .get_program_parameter(&program, GL::ACTIVE_ATTRIBUTES)
    .as_f64()
    .unwrap() as u32;
  let mut map = HashMap::new();
  for index in 0..num_attributes {
    let info = gl.get_active_attrib(&program, index).unwrap();
    let (size, type_) = match info.type_() {
      GL::FLOAT_VEC3 => (3, GL::FLOAT),
      GL::FLOAT_VEC2 => (2, GL::FLOAT),
      x => panic!("No match for attribute type: {}", x),
    };
    map.insert(
      info.name(),
      Attribute {
        name: info.name(),
        index,
        size,
        type_,
      },
    );
  }
  map
}

fn get_uniform_locations(gl: &GL, program: &WebGlProgram) -> HashMap<String, WebGlUniformLocation> {
  let num_uniforms = gl
    .get_program_parameter(&program, GL::ACTIVE_UNIFORMS)
    .as_f64()
    .unwrap() as u32;
  let mut map = HashMap::new();
  for i in 0..num_uniforms {
    let name = gl.get_active_uniform(&program, i).unwrap().name();
    let location = gl.get_uniform_location(&program, &name).unwrap();
    map.insert(name, location);
  }
  map
}
