use crate::{Engine, ObjectSettings, Vertex};

pub fn load_gltf<'a>(
    name: &'static str,
    path: &'static str,
    engine: &mut Engine,
) -> anyhow::Result<()> {
    let mut verticies = Vec::<Vertex>::new();
    let mut indicies = Vec::<u16>::new();

    let (gltf, buffers, _) = gltf::import(path)?;
    for mesh in gltf.meshes() {
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
    }

    engine.new_object(name, verticies, indicies, ObjectSettings::default())?;

    Ok(())
}
