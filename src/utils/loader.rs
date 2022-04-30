use crate::{Engine, Object, ObjectSettings, Vertex};

pub fn load_gltf<'a>(path: &'static str, engine: &'a mut Engine) -> anyhow::Result<&'a mut Object> {
    let mut verticies = Vec::<Vertex>::new();
    let mut indicies = Vec::<u16>::new();

    let (gltf, buffers, _) = gltf::import(path)?;
    for mesh in gltf.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(iter) = reader.read_positions() {
                for vertex_position in iter {
                    verticies.push(Vertex {
                        position: vertex_position,
                        texture: [0f32, 0f32],
                    });
                }
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
    }

    let model = engine.new_object(verticies, indicies, ObjectSettings::default())?;

    Ok(model)
}
