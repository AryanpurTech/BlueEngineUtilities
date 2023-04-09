use blue_engine::{ObjectSettings, ObjectStorage, Renderer, StringBuffer, Vertex};

#[cfg(feature = "animation")]
pub fn load_gltf<'a>(
    path: impl StringBuffer,
    renderer: &mut Renderer,
    objects: &mut ObjectStorage,
) -> anyhow::Result<()> {
    println!("THE MODEL LOADING FEATURE IS STILL EXPERIMENTAL!");
    println!("start parsing gltf");
    let (gltf, buffers, images) = gltf::import(path.as_str())?;

    let texture = renderer.build_texture(
        "text",
        blue_engine::TextureData::Bytes(images[0].pixels.clone()),
        blue_engine::TextureMode::Clamp,
    );

    println!("gltf parsed, starting disassembly");
    for mesh in gltf.meshes() {
        let mut verticies = Vec::<Vertex>::new();
        let mut indicies = Vec::<u16>::new();
        println!("{:?}", mesh.name());
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let mut positions: Vec<[f32; 3]> = Vec::new();
            if let Some(iter) = reader.read_positions() {
                for vertex_position in iter {
                    positions.push(vertex_position);
                }
            }

            let mut normals: Vec<[f32; 3]> = Vec::new();
            if let Some(iter) = reader.read_normals() {
                for normal in iter {
                    normals.push(normal);
                }
            }

            for i in 0..positions.len() {
                verticies.push(Vertex {
                    position: positions[i],
                    uv: [0f32, 0f32],
                    normal: normals[i],
                })
            }

            if let Some(index) = reader.read_indices() {
                match index {
                    gltf::mesh::util::ReadIndices::U16(iter) => {
                        for i in iter {
                            indicies.push(i);
                        }
                    }
                    gltf::mesh::util::ReadIndices::U32(iter) => {
                        for i in iter {
                            indicies.push(i as u16);
                        }
                    }
                    _ => (),
                }
            }
        }
        //break;
        objects.new_object(
            mesh.name()
                .unwrap_or(format!("{}:no_name", path.as_str()).as_str()),
            verticies,
            indicies,
            ObjectSettings::default(),
            renderer,
        )?;
    }

    Ok(())
}
