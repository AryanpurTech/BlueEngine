use wgpu::util::DeviceExt;

use crate::header::{Vertex, VertexBuffers};

impl crate::header::Renderer {
    /// Creates and adds the vertex buffers to render queue
    pub fn build_and_append_vertex_buffers(
        &mut self,
        verticies: Vec<Vertex>,
        indicies: Vec<u16>,
    ) -> Result<usize, anyhow::Error> {
        let vertex_buffers = self
            .build_vertex_buffers(verticies, indicies)
            .expect("Couldn't create vertex buffer");
        let index = self.vertex_buffers.len();
        self.vertex_buffers.push(vertex_buffers);
        Ok(index)
    }

    /// Creates a new vertex buffer and indecies
    pub fn build_vertex_buffers(
        &mut self,
        verticies: Vec<Vertex>,
        indicies: Vec<u16>,
    ) -> Result<VertexBuffers, anyhow::Error> {
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(verticies.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indicies.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            });

        Ok(VertexBuffers {
            vertex_buffer,
            index_buffer,
            length: indicies.len() as u32,
        })
    }

    /// Appends a vertex buffer to render queue
    pub fn append_vertex_buffer(
        &mut self,
        vertex_buffer: VertexBuffers,
    ) -> Result<usize, anyhow::Error> {
        let index = self.vertex_buffers.len();
        self.vertex_buffers.push(vertex_buffer);
        Ok(index)
    }

    /// Allows to modify a vertex buffer
    pub fn get_vertex_buffer(&mut self, index: usize) -> Result<&mut VertexBuffers, anyhow::Error> {
        Ok(self.vertex_buffers.get_mut(index).unwrap())
    }

    /// Removes vertex and index buffer group
    pub fn remove_vertex_buffer(&mut self, index: usize) -> Result<(), anyhow::Error> {
        self.vertex_buffers.remove(index);
        Ok(())
    }
}