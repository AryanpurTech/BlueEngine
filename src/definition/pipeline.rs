/*
 * Blue Engine by Elham Aryanpur
 *
 * 
 *
 * The license is same as the one on the root.
*/

use crate::header::Pipeline;

impl crate::header::Renderer {
    /// Creates and adds the pipeline to render queue
    pub fn build_and_append_pipeline(
        &mut self,
        shader_index: usize,
        vertex_buffer_index: usize,
        texture_index: usize,
        uniform_index: Option<usize>,
    ) -> Result<usize, anyhow::Error> {
        let pipe = self
            .build_pipeline(
                shader_index,
                vertex_buffer_index,
                texture_index,
                uniform_index,
            )
            .expect("Couldn't Create Render Pipeline");
        self.render_pipelines.push(pipe);
        Ok(self.render_pipelines.len() - 1)
    }

    /// Creates a new render pipeline. Could be thought of as like materials in game engines.
    pub fn build_pipeline(
        &mut self,
        shader_index: usize,
        vertex_buffer_index: usize,
        texture_index: usize,
        uniform_index: Option<usize>,
    ) -> Result<Pipeline, anyhow::Error> {
        Ok(Pipeline {
            shader_index,
            vertex_buffer_index,
            texture_index,
            uniform_index,
        })
    }

    /// Appends a pipeline to render queue
    pub fn append_pipeline(&mut self, pipeline: Pipeline) -> Result<usize, anyhow::Error> {
        self.render_pipelines.push(pipeline);
        Ok(self.render_pipelines.len() - 1)
    }

    /// Allows to modify a pipeline
    pub fn get_pipeline(&mut self, index: usize) -> Result<&mut Pipeline, anyhow::Error> {
        Ok(self.render_pipelines.get_mut(index).unwrap())
    }

    /// Deletes a render pipeline
    pub fn remove_pipeline(&mut self, index: usize) -> Result<(), anyhow::Error> {
        self.render_pipelines.remove(index);
        Ok(())
    }
}
