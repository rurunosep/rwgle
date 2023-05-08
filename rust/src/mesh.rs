use crate::renderer::Attribute;
use crate::utils;
use gltf::Gltf;
use nalgebra_glm as na;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlVertexArrayObject};

pub struct Mesh {
    vao: WebGlVertexArrayObject,
    index_buffer: WebGlBuffer,
    num_indices: i32,
}

impl Mesh {
    // TODO: If I'm using spawn_local on the async stuff below, does this "new"
    // and everything upstream have to be async at all?
    // I'm not solid on how all of this works. I can experiment, probably.
    pub async fn new(gl: &GL, attributes: &HashMap<String, Attribute>, gltf_url: &str) -> Mesh {
        // TODO:
        // // ----- Load the placeholder model data here
        // let s = String::from(gltf_url);
        // wasm_bindgen_futures::spawn_local(async move {
        //   let gltf = fetch_resource_as_array_buffer(&s).await;
        //   // ----- Load the gltf model data here
        // });

        let gltf = utils::fetch_resource_as_array_buffer(gltf_url).await;
        let gltf = Gltf::from_slice(js_sys::Uint8Array::new(&gltf).to_vec().as_slice()).unwrap();

        let buffer = if let gltf::buffer::Source::Uri(uri) = gltf.buffers().next().unwrap().source()
        {
            utils::fetch_resource_as_array_buffer(uri).await
        } else {
            panic!("Can only read gltf buffer from url currently")
        };

        let mesh = gltf.meshes().next().unwrap();
        let primitive = mesh.primitives().next().unwrap();
        let positions = {
            let accessor = primitive.get(&gltf::mesh::Semantic::Positions).unwrap();
            let view = accessor.view().unwrap();
            js_sys::Float32Array::new_with_byte_offset_and_length(
                &buffer,
                view.offset() as u32,
                view.length() as u32 / 4,
            )
        };
        let normals = {
            let accessor = primitive.get(&gltf::mesh::Semantic::Normals).unwrap();
            let view = accessor.view().unwrap();
            js_sys::Float32Array::new_with_byte_offset_and_length(
                &buffer,
                view.offset() as u32,
                view.length() as u32 / 4,
            )
        };
        let texcoords = {
            let accessor = primitive.get(&gltf::mesh::Semantic::TexCoords(0)).unwrap();
            let view = accessor.view().unwrap();
            js_sys::Float32Array::new_with_byte_offset_and_length(
                &buffer,
                view.offset() as u32,
                view.length() as u32 / 4,
            )
        };
        let indices = {
            let accessor = primitive.indices().unwrap();
            let view = accessor.view().unwrap();
            js_sys::Uint16Array::new_with_byte_offset_and_length(
                &buffer,
                view.offset() as u32,
                view.length() as u32 / 2,
            )
        };

        let (tangents, bitangents) = calc_tangents_bitangents(&indices, &positions, &texcoords);

        let (index_buffer, num_indices) = buffer_index_data(&gl, &indices);

        let oesvao = gl
            .get_extension("OES_vertex_array_object")
            .unwrap()
            .unwrap()
            .unchecked_into::<web_sys::OesVertexArrayObject>();

        let vao = oesvao.create_vertex_array_oes().unwrap();
        oesvao.bind_vertex_array_oes(Some(&vao));

        buffer_and_set_pointer(&gl, attributes.get("a_position").unwrap(), &positions);
        buffer_and_set_pointer(&gl, attributes.get("a_texcoords").unwrap(), &texcoords);
        buffer_and_set_pointer(&gl, attributes.get("a_normal").unwrap(), &normals);
        buffer_and_set_pointer(&gl, attributes.get("a_tangent").unwrap(), &tangents);
        buffer_and_set_pointer(&gl, attributes.get("a_bitangent").unwrap(), &bitangents);

        Mesh {
            vao,
            index_buffer,
            num_indices,
        }
    }

    pub fn render(&self, gl: &GL) {
        let oesvao = gl
            .get_extension("OES_vertex_array_object")
            .unwrap()
            .unwrap()
            .unchecked_into::<web_sys::OesVertexArrayObject>();
        oesvao.bind_vertex_array_oes(Some(&self.vao));

        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));

        gl.draw_elements_with_i32(GL::TRIANGLES, self.num_indices, GL::UNSIGNED_SHORT, 0);
    }
}

fn buffer_and_set_pointer(gl: &GL, attrib: &Attribute, array: &js_sys::Float32Array) {
    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
    gl.vertex_attrib_pointer_with_i32(attrib.index, attrib.size, attrib.type_, false, 0, 0);
    gl.enable_vertex_attrib_array(attrib.index);
}

fn buffer_index_data(gl: &GL, array: &js_sys::Uint16Array) -> (WebGlBuffer, i32) {
    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &array, GL::STATIC_DRAW);
    (buffer, array.length() as i32)
}

// TODO: nicer
pub fn calc_tangents_bitangents(
    indices: &js_sys::Uint16Array,
    positions: &js_sys::Float32Array,
    texcoords: &js_sys::Float32Array,
) -> (js_sys::Float32Array, js_sys::Float32Array) {
    let indices = indices.to_vec();
    let indices = indices.as_slice();
    let positions = positions.to_vec();
    let positions = positions.as_slice();
    let texcoords = texcoords.to_vec();
    let texcoords = texcoords.as_slice();

    let mut tangents: Vec<f32> = Vec::with_capacity(positions.len() as usize);
    tangents.resize(positions.len(), 0.);
    let mut bitangents: Vec<f32> = Vec::with_capacity(positions.len() as usize);
    bitangents.resize(positions.len(), 0.);

    for i in (0..indices.len()).step_by(3) {
        let v0 = na::vec3(
            positions[indices[i + 0] as usize * 3 + 0],
            positions[indices[i + 0] as usize * 3 + 1],
            positions[indices[i + 0] as usize * 3 + 2],
        );
        let v1 = na::vec3(
            positions[indices[i + 1] as usize * 3 + 0],
            positions[indices[i + 1] as usize * 3 + 1],
            positions[indices[i + 1] as usize * 3 + 2],
        );
        let v2 = na::vec3(
            positions[indices[i + 2] as usize * 3 + 0],
            positions[indices[i + 2] as usize * 3 + 1],
            positions[indices[i + 2] as usize * 3 + 2],
        );

        let uv0 = na::vec2(
            texcoords[indices[i + 0] as usize * 2 + 0],
            texcoords[indices[i + 0] as usize * 2 + 1],
        );
        let uv1 = na::vec2(
            texcoords[indices[i + 1] as usize * 2 + 0],
            texcoords[indices[i + 1] as usize * 2 + 1],
        );
        let uv2 = na::vec2(
            texcoords[indices[i + 2] as usize * 2 + 0],
            texcoords[indices[i + 2] as usize * 2 + 1],
        );

        let delta_pos_1 = v1 - v0;
        let delta_pos_2 = v2 - v0;

        let delta_uv_1 = uv1 - uv0;
        let delta_uv_2 = uv2 - uv0;

        let r = 1.0 / (delta_uv_1.x * delta_uv_2.y - delta_uv_1.y * delta_uv_2.x);
        let tangent = (delta_pos_1 * delta_uv_2.y - delta_pos_2 * delta_uv_1.y) * r;
        let bitangent = (delta_pos_2 * delta_uv_1.x - delta_pos_1 * delta_uv_2.x) * r;

        tangents[indices[i + 0] as usize * 3 + 0] = tangent.x;
        tangents[indices[i + 0] as usize * 3 + 1] = tangent.y;
        tangents[indices[i + 0] as usize * 3 + 2] = tangent.z;
        tangents[indices[i + 1] as usize * 3 + 0] = tangent.x;
        tangents[indices[i + 1] as usize * 3 + 1] = tangent.y;
        tangents[indices[i + 1] as usize * 3 + 2] = tangent.z;
        tangents[indices[i + 2] as usize * 3 + 0] = tangent.x;
        tangents[indices[i + 2] as usize * 3 + 1] = tangent.y;
        tangents[indices[i + 2] as usize * 3 + 2] = tangent.z;

        bitangents[indices[i + 0] as usize * 3 + 0] = bitangent.x;
        bitangents[indices[i + 0] as usize * 3 + 1] = bitangent.y;
        bitangents[indices[i + 0] as usize * 3 + 2] = bitangent.z;
        bitangents[indices[i + 1] as usize * 3 + 0] = bitangent.x;
        bitangents[indices[i + 1] as usize * 3 + 1] = bitangent.y;
        bitangents[indices[i + 1] as usize * 3 + 2] = bitangent.z;
        bitangents[indices[i + 2] as usize * 3 + 0] = bitangent.x;
        bitangents[indices[i + 2] as usize * 3 + 1] = bitangent.y;
        bitangents[indices[i + 2] as usize * 3 + 2] = bitangent.z;
    }

    unsafe {
        let tangents = js_sys::Float32Array::view(&tangents);
        let bitangents = js_sys::Float32Array::view(&bitangents);
        (tangents, bitangents)
    }
}
