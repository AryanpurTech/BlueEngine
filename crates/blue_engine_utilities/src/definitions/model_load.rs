#![cfg(feature = "gltf")]

use blue_engine::{Object, ObjectSettings, ObjectStorage, Renderer, Vertex};

pub fn load_gltf(
    name: Option<impl AsRef<str>>,
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
                    position: positions[i],
                    uv: [0f32, 0f32],
                    normal: normals[i],
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
        let name = if let Some(ref new_name) = name {
            new_name.as_ref()
        } else if let Some(new_name) = mesh.name() {
            new_name
        } else {
            path.to_str().unwrap()
        };

        //break;
        objects.insert(
            name.into(),
            Object::new(
                name,
                verticies,
                indicies,
                ObjectSettings::default(),
                renderer,
            )?,
        );
    }

    Ok(())
}

#[cfg(feature = "obj")]
pub fn load_obj(
    name: Option<impl AsRef<str>>,
    path: &std::path::Path,
    renderer: &mut Renderer,
    objects: &mut ObjectStorage,
) -> eyre::Result<()> {
    use std::io::BufReader;

    let buffer = BufReader::new(std::fs::File::open(path)?);
    let model_desc: obj::Obj<obj::TexturedVertex> = obj::load_obj(buffer)?;
    let name = if let Some(ref name) = name {
        name.as_ref()
    } else {
        path.to_str().unwrap()
    };

    objects.insert(
        name.into(),
        Object::new(
            name,
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
        )?,
    );

    Ok(())
}
