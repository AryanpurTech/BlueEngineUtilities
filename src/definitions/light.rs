use blue_engine::{uniform_type, ObjectStorage, Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LightUniforms {
    light_color: uniform_type::Array4,     // 4 units
    light_position: uniform_type::Array3,  // 3 units
    ambient_strength: f32,                 // 1 unit
    camera_position: uniform_type::Array3, // 3 units
    specular_strength: f32,                // 1 unit
    inverse_model: uniform_type::Matrix,   // 4x4 units
}
unsafe impl Pod for LightUniforms {}
unsafe impl Zeroable for LightUniforms {}

impl crate::LightManager {
    pub fn new() -> Self {
        Self {
            ambient_color: uniform_type::Array4 {
                data: [1f32, 1f32, 1f32, 1f32], //0.051f32, 0.533f32, 0.898f32
            },
            ambient_strength: 0f32,
            affected_objects: Vec::new(),
            light_objects: std::collections::BTreeMap::new(),
        }
    }

    pub fn update(
        &mut self,
        objects: &mut ObjectStorage,
        renderer: &mut blue_engine::Renderer,
        camera: &blue_engine::Camera,
    ) -> color_eyre::Result<()> {
        let light_keys: Vec<String> = self.light_objects.keys().map(|x| x.clone()).collect();
        let shader_content = include_str!("./light_shader.wgsl").to_string();

        for i in objects.iter_mut() {
            let i = i.1;
            if light_keys.contains(&i.name) {
                self.light_objects.insert(
                    i.name.clone(),
                    ([i.position.x, i.position.y, i.position.z], i.color),
                );
            } else {
                let result = i.color * self.ambient_color;
                i.set_uniform_color(
                    result.data[0],
                    result.data[1],
                    result.data[2],
                    result.data[3],
                )?;

                let pos = *self.light_objects.get(&light_keys[0]).unwrap();
                let light_uniform_buffer = renderer.build_uniform_buffer_part(
                    "light_uniform_buffer",
                    LightUniforms {
                        light_color: pos.1,
                        light_position: uniform_type::Array3 {
                            data: [pos.0[0], pos.0[1], pos.0[2]],
                        },
                        ambient_strength: self.ambient_strength,
                        inverse_model: i.inverse_transformation_matrix,
                        camera_position: uniform_type::Array3 {
                            data: camera.position.data.0[0],
                        },
                        specular_strength: 0.8,
                    },
                );
                if i.uniform_buffers.len() == 2 {
                    i.uniform_buffers.push(light_uniform_buffer);
                } else {
                    i.uniform_buffers[2] = light_uniform_buffer;
                }

                i.update_uniform_buffer(renderer)?;

                let mut shader_content = shader_content.clone();

                if !self.affected_objects.contains(&i.name) {
                    if i.camera_effect {
                        shader_content = shader_content.replace(
                            "//@CAMERASTRUCT",
                            r#"
struct CameraUniforms {
    camera_matrix: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera_uniform: CameraUniforms;"#,
                        );
                        shader_content = shader_content.replace("//@CAMERAOUT", "out.position = camera_uniform.camera_matrix * (transform_uniform.transform_matrix * vec4<f32>(input.position, 1.0));");
                    } else {
                        shader_content = shader_content.replace("//@CAMERAOUT","out.position = transform_uniform.transform_matrix * vec4<f32>(input.position, 1.0);");
                    }

                    i.shader_builder.shader = shader_content;
                    i.update_shader(renderer)?;

                    self.affected_objects.push(i.name.clone());
                }
            }
        }

        Ok(())
    }

    pub fn set_object_as_light(&mut self, object: String) {
        self.light_objects.insert(
            object,
            (
                [0f32, 0f32, 0f32],
                uniform_type::Array4 {
                    data: [0f32, 0f32, 0f32, 0f32],
                },
            ),
        );
    }
}
