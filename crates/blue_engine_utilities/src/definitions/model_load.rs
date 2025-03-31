#![cfg(feature = "gltf")]

use blue_engine::{ObjectSettings, ObjectStorage, Renderer, StringBuffer, Vertex};

pub fn load_gltf(
    name: Option<impl StringBuffer>,
    path: &std::path::Path,
    renderer: &mut Renderer,
    objects: &mut ObjectStorage,
) -> eyre::Result<()> {
    use blue_engine::UnsignedIntType;

    println!("THE MODEL LOADING FEATURE IS STILL EXPERIMENTAL!");
    println!("start parsing gltf");
    let (gltf, buffers, images) = gltf::import(path)?;

    let mut _texture: Option<blue_engine::Textures> = None;
    if !images.is_empty() {
        _texture = Some(renderer.build_texture(
            "text",
            blue_engine::TextureData::Bytes(images[0].pixels.clone()),
            blue_engine::TextureMode::Clamp,
        )?);
    }

    println!("gltf parsed, starting disassembly");
    for mesh in gltf.meshes() {
        let mut verticies = Vec::<Vertex>::new();
        let mut indicies = Vec::new();
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
                    position: positions[i].into(),
                    uv: [0f32, 0f32].into(),
                    normal: normals[i].into(),
                })
            }

            if let Some(index) = reader.read_indices() {
                match index {
                    gltf::mesh::util::ReadIndices::U16(iter) => {
                        for i in iter {
                            indicies.push(i as UnsignedIntType);
                        }
                    }
                    gltf::mesh::util::ReadIndices::U32(iter) => {
                        for i in iter {
                            indicies.push(i as UnsignedIntType);
                        }
                    }
                    _ => (),
                }
            }
        }
        //break;
        objects.new_object(
            if name.as_ref().is_some() {
                let new_name = name.as_ref().unwrap();
                new_name.as_str()
            } else if mesh.name().is_some() {
                mesh.name().unwrap()
            } else {
                path.to_str().unwrap()
            },
            verticies,
            indicies,
            ObjectSettings::default(),
            renderer,
        );
    }

    Ok(())
}

#[cfg(feature = "obj")]
pub fn load_obj(
    name: Option<impl StringBuffer>,
    path: &std::path::Path,
    renderer: &mut Renderer,
    objects: &mut ObjectStorage,
) -> eyre::Result<()> {
    use std::io::BufReader;

    let buffer = BufReader::new(std::fs::File::open(path)?);
    let model_desc: obj::Obj<obj::TexturedVertex> = obj::load_obj(buffer)?;
    objects.new_object(
        if name.as_ref().is_some() {
            let new_name = name.as_ref().unwrap();
            new_name.as_str()
        } else {
            path.to_str().unwrap()
        },
        model_desc
            .vertices
            .iter()
            .map(|vertex| {
                let pos = vertex.position;
                let norm = vertex.normal;
                let uv = [vertex.texture[0], vertex.texture[1]];
                blue_engine::Vertex {
                    position: pos,
                    uv,
                    normal: norm,
                }
            })
            .collect(),
        model_desc.indices,
        blue_engine::ObjectSettings {
            ..Default::default()
        },
        renderer,
    )?;

    Ok(())
}
