use crate::{ObjectSettings, ObjectStorage, Renderer, StringBuffer, Vertex};

/// Creates a 3D cube
pub fn cube(
    name: impl StringBuffer,
    renderer: &mut Renderer,
    objects: &mut ObjectStorage,
) -> color_eyre::Result<()> {
    objects.new_object(
        name.clone(),
        vec![
            // Front Face
            Vertex {
                position: [-1.0, -1.0, 1.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [1.0, -1.0, 1.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [-1.0, 1.0, 1.0],
                uv: [0.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            // Back Face
            Vertex {
                position: [-1.0, 1.0, -1.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [1.0, 1.0, -1.0],
                uv: [0.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [1.0, -1.0, -1.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [-1.0, -1.0, -1.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            // Right face
            Vertex {
                position: [1.0, -1.0, -1.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [1.0, 1.0, -1.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
                uv: [0.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [1.0, -1.0, 1.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            // Left Face
            Vertex {
                position: [-1.0, -1.0, 1.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [-1.0, 1.0, 1.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [-1.0, 1.0, -1.0],
                uv: [0.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [-1.0, -1.0, -1.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            // Top Face
            Vertex {
                position: [1.0, 1.0, -1.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [-1.0, 1.0, -1.0],
                uv: [0.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [-1.0, 1.0, 1.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            // Bottom Face
            Vertex {
                position: [1.0, -1.0, 1.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [-1.0, -1.0, 1.0],
                uv: [0.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [-1.0, -1.0, -1.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [1.0, -1.0, -1.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
        ],
        vec![
            0, 1, 2, 2, 3, 0, // top
            4, 5, 6, 6, 7, 4, // bottom
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // front
            20, 21, 22, 22, 23, 20, // back
        ],
        ObjectSettings::default(),
        renderer,
    )?;

    Ok(())
}

/// Create a 3D UV Sphere
///
/// ```
/// details = (stacks, sectors, radius)
/// example = (18, 36, 1f32)
/// ```
pub fn uv_sphere(
    name: impl StringBuffer,
    details: (usize, usize, f32),
    renderer: &mut Renderer,
    objects: &mut ObjectStorage,
) -> color_eyre::Result<()> {
    let sectors = details.1 as f32;
    let stacks = details.0 as f32;
    let length_inv = 1. / details.2;
    let sector_step = 2. * std::f32::consts::PI / sectors;
    let stack_step = std::f32::consts::PI / stacks;

    let mut vertices: Vec<Vertex> = Vec::with_capacity(details.0 * details.1);
    let mut indices: Vec<u16> = Vec::with_capacity(details.0 * details.1 * 2 * 3);

    for i in 0..details.0 + 1 {
        let stack_angle = std::f32::consts::PI / 2. - (i as f32) * stack_step;
        let xy = details.2 * stack_angle.cos();
        let z = details.2 * stack_angle.sin();

        for j in 0..details.1 + 1 {
            let sector_angle = (j as f32) * sector_step;
            let x = xy * sector_angle.cos();
            let y = xy * sector_angle.sin();

            vertices.push(Vertex {
                position: [x, y, z],
                uv: [(j as f32) / sectors, (i as f32) / stacks],
                normal: [x * length_inv, y * length_inv, z * length_inv],
            });
        }
    }
    for i in 0..details.0 {
        let mut k1 = i * (details.1 + 1);
        let mut k2 = k1 + details.1 + 1;
        for _j in 0..details.1 {
            if i != 0 {
                indices.push(k1 as u16);
                indices.push(k2 as u16);
                indices.push((k1 + 1) as u16);
            }
            if i != details.0 - 1 {
                indices.push((k1 + 1) as u16);
                indices.push(k2 as u16);
                indices.push((k2 + 1) as u16);
            }
            k1 += 1;
            k2 += 1;
        }
    }

    objects.new_object(
        name.clone(),
        vertices,
        indices,
        ObjectSettings::default(),
        renderer,
    )?;

    Ok(())
}
