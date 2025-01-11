use crate::{
    ObjectSettings, ObjectStorage, Renderer, StringBuffer, UnsignedIntType, Vector2, Vector3,
    Vertex,
};
use std::f32::consts::PI;

/// Creates a 3D cube
pub fn cube(name: impl StringBuffer, renderer: &mut Renderer, objects: &mut ObjectStorage) {
    objects.new_object(
        name.clone(),
        vec![
            // Front Face
            Vertex {
                position: Vector3::new(-1.0, -1.0, 1.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, -1.0, 1.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, 1.0, 1.0),
                uv: Vector2::new(1.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, 1.0, 1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            // Back Face
            Vertex {
                position: Vector3::new(-1.0, 1.0, -1.0),
                uv: Vector2::new(1.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, 1.0, -1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, -1.0, -1.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, -1.0, -1.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            // Right face
            Vertex {
                position: Vector3::new(1.0, -1.0, -1.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, 1.0, -1.0),
                uv: Vector2::new(1.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, 1.0, 1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, -1.0, 1.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            // Left face
            Vertex {
                position: Vector3::new(-1.0, -1.0, 1.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, 1.0, 1.0),
                uv: Vector2::new(1.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, 1.0, -1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, -1.0, -1.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            // Top face
            Vertex {
                position: Vector3::new(1.0, 1.0, -1.0),
                uv: Vector2::new(1.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, 1.0, -1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, 1.0, 1.0),
                uv: Vector2::new(0.0, -1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, 1.0, 1.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            // Bottom face
            Vertex {
                position: Vector3::new(1.0, -1.0, 1.0),
                uv: Vector2::new(1.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, -1.0, 1.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, -1.0, -1.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, -1.0, 1.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
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
    );
}

/// Create a 3D UV Sphere
///
/// ```
/// details = (stacks, sectors, radius)
/// example = (18, 36, 1.0)
/// ```
pub fn uv_sphere(
    name: impl StringBuffer,
    details: (usize, usize, f32),
    renderer: &mut Renderer,
    objects: &mut ObjectStorage,
) {
    let sectors = details.1 as f32;
    let stacks = details.0 as f32;
    let length_inv = 1. / details.2;
    let sector_step = 2. * PI / sectors;
    let stack_step = PI / stacks;

    let mut vertices: Vec<Vertex> = Vec::with_capacity(details.0 * details.1);
    let mut indices: Vec<UnsignedIntType> = Vec::with_capacity(details.0 * details.1 * 2 * 3);

    for i in 0..details.0 + 1 {
        let stack_angle = PI / 2.0 - (i as f32) * stack_step;
        let xy: f32 = details.2 * stack_angle.cos();
        let z: f32 = details.2 * stack_angle.sin();

        for j in 0..details.1 + 1 {
            let sector_angle = (j as f32) * sector_step;
            let x: f32 = xy * sector_angle.cos();
            let y: f32 = xy * sector_angle.sin();

            vertices.push(Vertex {
                position: [x, y, z].into(),
                uv: Vector2::new((j as f32) / sectors, (i as f32) / stacks),
                normal: Vector3::new(x * length_inv, y * length_inv, z * length_inv),
            });
        }
    }
    for i in 0..details.0 {
        let mut k1 = i * (details.1 + 1);
        let mut k2 = k1 + details.1 + 1;
        for _j in 0..details.1 {
            if i != 0 {
                indices.push(k1 as UnsignedIntType);
                indices.push(k2 as UnsignedIntType);
                indices.push((k1 + 1) as UnsignedIntType);
            }
            if i != details.0 - 1 {
                indices.push((k1 + 1) as UnsignedIntType);
                indices.push(k2 as UnsignedIntType);
                indices.push((k2 + 1) as UnsignedIntType);
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
    );
}
