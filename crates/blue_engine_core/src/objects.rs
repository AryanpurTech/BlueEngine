/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::prelude::{
    Instance, InstanceRaw, Object, ObjectSettings, Pipeline, PipelineData, Renderer, RotateAxis,
    TextureData, Textures, Vertex, glm, pixel_to_cartesian, uniform_type,
};
use crate::uniform_type::{Array4, Matrix};
use crate::utils::default_resources::{DEFAULT_MATRIX_4, DEFAULT_SHADER, DEFAULT_TEXTURE};
use crate::{ObjectStorage, RotateAmount, StringBuffer, UnsignedIntType, Vector3};

impl Renderer {
    /// Creates a new object
    ///
    /// Is used to define a new object and add it to the storage. This offers full customizability
    /// and a framework for in-engine shapes to be developed.
    ///
    /// # Arguments
    /// * `name` - The name of the object.
    /// * `vertices` - A list of vertices for the object to draw with
    /// * `indices` - A list of indices that references the vertices, defining draw order
    /// * `settings` - The settings of the object
    pub fn build_object(
        &mut self,
        name: impl StringBuffer,
        vertices: Vec<Vertex>,
        indices: Vec<UnsignedIntType>,
        settings: ObjectSettings,
    ) -> Result<Object, crate::error::Error> {
        let vertex_buffer = self.build_vertex_buffer(&vertices, &indices);

        let uniform = self.build_uniform_buffer(&vec![
            self.build_uniform_buffer_part("Transformation Matrix", DEFAULT_MATRIX_4),
            self.build_uniform_buffer_part(
                "Color",
                crate::uniform_type::Array4 {
                    data: crate::utils::default_resources::DEFAULT_COLOR,
                },
            ),
        ]);

        let shader_source =
            ShaderBuilder::new(DEFAULT_SHADER.to_string(), settings.camera_effect.clone());

        let shader = self.build_shader(
            name.as_str(),
            shader_source.shader.clone(),
            Some(&uniform.1),
            settings.shader_settings,
        );

        let texture = self.build_texture(
            "Default Texture",
            TextureData::Bytes(DEFAULT_TEXTURE.to_vec()),
            crate::prelude::TextureMode::Clamp,
            //crate::prelude::TextureFormat::PNG
        )?;

        let instance = Instance::new([0f32, 0f32, 0f32], [0f32, 0f32, 0f32], [1f32, 1f32, 1f32]);

        let instance_buffer = self.build_instance(vec![instance.to_raw()]);

        Ok(Object {
            name: name.as_arc(),
            vertices,
            indices,
            pipeline: Pipeline {
                vertex_buffer: PipelineData::Data(vertex_buffer),
                shader: PipelineData::Data(shader),
                texture: PipelineData::Data(texture),
                uniform: PipelineData::Data(Some(uniform.0)),
            },
            instances: vec![instance],
            instance_buffer,
            uniform_layout: uniform.1,
            size: Vector3::new(1f32, 1f32, 1f32),
            position: Vector3::default(),
            rotation: Vector3::new(0f32, 0f32, 0f32),
            changed: false,
            position_matrix: DEFAULT_MATRIX_4.to_im(),
            scale_matrix: DEFAULT_MATRIX_4.to_im(),
            rotation_matrix: DEFAULT_MATRIX_4.to_im(),
            inverse_transformation_matrix: Matrix::from_im(nalgebra_glm::transpose(
                &nalgebra_glm::inverse(&DEFAULT_MATRIX_4.to_im()),
            )),
            color: crate::uniform_type::Array4 {
                data: crate::utils::default_resources::DEFAULT_COLOR,
            },
            shader_builder: shader_source,
            shader_settings: settings.shader_settings,
            camera_effect: settings.camera_effect,
            uniform_buffers: vec![
                self.build_uniform_buffer_part("Transformation Matrix", DEFAULT_MATRIX_4),
                self.build_uniform_buffer_part(
                    "Color",
                    crate::uniform_type::Array4 {
                        data: crate::utils::default_resources::DEFAULT_COLOR,
                    },
                ),
            ],
            is_visible: true,
            render_order: 0,
        })
    }
}

impl ObjectStorage {
    /// Creates a new object
    pub fn new_object(
        &mut self,
        name: impl StringBuffer,
        vertices: Vec<Vertex>,
        indices: Vec<UnsignedIntType>,
        settings: ObjectSettings,
        renderer: &mut Renderer,
    ) {
        match renderer.build_object(name.clone(), vertices, indices, settings) {
            Ok(object) => self.add_object(name.clone(), object),
            Err(e) => {
                eprintln!("Could not create a new Object: {e:#?}");
            }
        }
    }

    /// Adds an object to the storage
    pub fn add_object(&mut self, key: impl StringBuffer, object: Object) {
        fn add_object_inner(object_storage: &mut ObjectStorage, key: String, object: Object) {
            object_storage.insert(key, object);
        }
        add_object_inner(self, key.as_string(), object);
    }

    /// Allows for safe update of objects
    pub fn update_object<T: Fn(&mut Object)>(&mut self, key: impl StringBuffer, callback: T) {
        fn update_object_inner<T: Fn(&mut Object)>(
            object_storage: &mut ObjectStorage,
            key: String,
            callback: T,
        ) {
            let object = object_storage.get_mut(&key);
            if let Some(object) = object {
                callback(object);
            }
        }
        update_object_inner(self, key.as_string(), callback);
    }
}

impl Object {
    /// Sets the name of the object
    pub fn set_name(&mut self, name: impl StringBuffer) -> &mut Self {
        self.name = name.as_arc();

        self
    }

    /// Scales an object. e.g. 2.0 doubles the size and 0.5 halves
    pub fn set_scale(&mut self, scale: impl Into<Vector3>) -> &mut Self {
        let scale = scale.into();
        self.size *= scale;

        let transformation_matrix = self.scale_matrix;
        let result = nalgebra_glm::scale(&transformation_matrix, &scale.into());
        self.scale_matrix = result;
        self.inverse_matrices();

        self.changed = true;
        self
    }

    /// Resizes an object in pixels which are relative to the window
    pub fn resize(
        &mut self,
        width: f32,
        height: f32,
        depth: f32,
        window_size: winit::dpi::PhysicalSize<u32>,
    ) -> &mut Self {
        let difference_in_width = if self.size.x != 0.0 && width != 0.0 {
            let a = pixel_to_cartesian(width, window_size.width);
            let b = pixel_to_cartesian(self.size.x, window_size.width);
            if a != 0f32 && b != 0f32 { a / b } else { b }
        } else {
            0.0
        };

        let difference_in_height = if self.size.y != 0.0 && height != 0.0 {
            let a = pixel_to_cartesian(height, window_size.height);
            let b = pixel_to_cartesian(self.size.y, window_size.height);
            if a != 0f32 && b != 0f32 { a / b } else { b }
        } else {
            0.0
        };
        let difference_in_depth = if self.size.z != 0.0 && depth != 0.0 {
            let a = pixel_to_cartesian(depth, window_size.width);
            let b = pixel_to_cartesian(self.size.z, window_size.width);
            if a != 0f32 && b != 0f32 { a / b } else { b }
        } else {
            0.0
        };

        self.set_scale(Vector3::new(
            difference_in_width,
            difference_in_height,
            difference_in_depth,
        ));
        self
    }

    /// Rotates the object in the axis you specify
    ///
    /// THIS METHOD IS DEPRECATED, USE [crate::Object::set_rotation] or [crate::Object::rotate]
    #[deprecated]
    pub fn set_rotatation(&mut self, angle: f32, axis: RotateAxis) -> &mut Self {
        let mut rotation_matrix = self.rotation_matrix;
        let axis = match axis {
            RotateAxis::X => {
                self.rotation.x += angle;
                Vector3::x_axis()
            }
            RotateAxis::Y => {
                self.rotation.y += angle;
                Vector3::y_axis()
            }
            RotateAxis::Z => {
                self.rotation.z += angle;
                Vector3::z_axis()
            }
        };

        rotation_matrix = nalgebra_glm::rotate(&rotation_matrix, angle.to_radians(), &axis.into());
        self.rotation_matrix = rotation_matrix;
        self.inverse_matrices();

        self.changed = true;
        self
    }

    fn rotation_single_axis(angle: f32, axis_from: usize, axis_into: usize) -> nalgebra_glm::Mat4 {
        //  angle.cos(), -angle.sin()
        //  angle.sin(), angle.cos()
        let mut result = nalgebra_glm::Mat4::identity();

        result[(axis_from, axis_from)] = angle.cos() as f32;
        result[(axis_from, axis_into)] = -angle.sin() as f32;
        result[(axis_into, axis_from)] = angle.sin() as f32;
        result[(axis_into, axis_into)] = angle.cos() as f32;

        result
    }
    fn rotation_full(euler_angles: nalgebra_glm::Vec3) -> nalgebra_glm::Mat4 {
        const X: usize = 0;
        const Y: usize = 1;
        const Z: usize = 2;

        Self::rotation_single_axis(euler_angles[2], X, Y) * // Rotation around Z (rotation of X into Y)
        Self::rotation_single_axis(euler_angles[1], Z, X) * // Rotation around Y (rotation of Z into X)
        Self::rotation_single_axis(euler_angles[0], Y, Z) // Rotation around X (rotation of Y into Z)
    }

    /// Sets the rotation of the object in the axis you specify
    pub fn set_rotation(&mut self, amount: RotateAmount, axis: RotateAxis) -> &mut Self {
        let amount_radians = match amount {
            RotateAmount::Radians(amount) => amount,
            RotateAmount::Degrees(amount) => amount.to_radians(),
        };
        match axis {
            RotateAxis::X => {
                self.rotation.x = amount_radians;
                Vector3::x_axis()
            }
            RotateAxis::Y => {
                self.rotation.y = amount_radians;
                Vector3::y_axis()
            }
            RotateAxis::Z => {
                self.rotation.z = amount_radians;
                Vector3::z_axis()
            }
        };

        self.rotation_matrix = Self::rotation_full(self.rotation.into());
        self.inverse_matrices();

        self.changed = true;
        self
    }

    /// Rotates the object in the axis you specify
    pub fn rotate(&mut self, amount: RotateAmount, axis: RotateAxis) -> &mut Self {
        let mut rotation_matrix = self.rotation_matrix;

        let amount_radians = match amount {
            RotateAmount::Radians(amount) => amount,
            RotateAmount::Degrees(amount) => amount.to_radians(),
        };
        let axis = match axis {
            RotateAxis::X => {
                self.rotation.x += amount_radians;
                Vector3::x_axis()
            }
            RotateAxis::Y => {
                self.rotation.y += amount_radians;
                Vector3::y_axis()
            }
            RotateAxis::Z => {
                self.rotation.z += amount_radians;
                Vector3::z_axis()
            }
        };

        rotation_matrix = nalgebra_glm::rotate(&rotation_matrix, amount_radians, &axis.into());
        self.rotation_matrix = rotation_matrix;
        self.inverse_matrices();

        self.changed = true;
        self
    }

    /// Moves the object by the amount you specify in the axis you specify
    pub fn set_translation(&mut self, new_pos: impl Into<Vector3>) -> &mut Self {
        self.position -= new_pos.into();

        let mut position_matrix = self.position_matrix;
        position_matrix = nalgebra_glm::translate(&position_matrix, &self.position.into());
        self.position_matrix = position_matrix;

        self.inverse_matrices();
        self.changed = true;
        self
    }

    /// Sets the position of the object in 3D space relative to the window
    pub fn set_position(&mut self, new_pos: impl Into<Vector3>) -> &mut Self {
        let new_pos = new_pos.into();

        self.position.x = new_pos.x;
        self.position.y = new_pos.y;
        self.position.z = new_pos.z;

        // self.set_translation((self.position - new_pos) * -1f32);
        self.update_position_matrix();
        self.inverse_matrices();
        self.changed = true;

        self
    }

    fn update_position_matrix(&mut self) {
        // If there was actual `nalgebra`, it could be just:
        // nalgebra::matrix![
        //     1.0,  0.0,  0.0,  shift[0];
        //     0.0,  1.0,  0.0,  shift[1];
        //     0.0,  0.0,  1.0,  shift[2];
        //     0.0,  0.0,  0.0,  1.0;
        // ]

        self.position_matrix = DEFAULT_MATRIX_4.to_im();
        self.position_matrix[(0, 3)] = self.position[0];
        self.position_matrix[(1, 3)] = self.position[1];
        self.position_matrix[(2, 3)] = self.position[2];
    }

    /// Changes the color of the object. If textures exist, the color of textures will change
    pub fn set_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) -> &mut Self {
        self.color = Array4 {
            data: [red, green, blue, alpha],
        };
        self.changed = true;
        self
    }

    /// Changes the render order of the Object.
    ///
    /// Objects with higher number get rendered later and appear "on top" when occupying the same space
    pub fn set_render_order(&mut self, render_order: usize) -> &mut Self {
        self.render_order = render_order;

        self
    }

    /// Replaces the object's texture with provided one
    pub fn set_texture(&mut self, texture: Textures) -> &mut Self {
        self.pipeline.texture = PipelineData::Data(texture);
        self.changed = true;

        self
    }

    /// This will flag object as changed and altered, leading to rebuilding parts, or entirety on next frame.
    /// Best used if you directly altered fields of the object. The functions normally flag the object as
    /// changed on every call anyways. But this function is to manually flag it yourself.
    pub fn flag_as_changed(&mut self, is_changed: bool) {
        self.changed = is_changed;
    }

    /// Sets if the object will be rendered or not
    pub fn set_visibility(&mut self, is_visible: bool) {
        self.is_visible = is_visible;
    }

    /// build an inverse of the transformation matrix to be sent to the gpu for lighting and other things.
    pub fn inverse_matrices(&mut self) {
        self.inverse_transformation_matrix =
            Matrix::from_im(nalgebra_glm::transpose(&nalgebra_glm::inverse(
                &(self.position_matrix * self.rotation_matrix * self.scale_matrix),
            )));
    }

    /// Update and apply changes done to an object
    pub fn update(&mut self, renderer: &mut Renderer) {
        self.update_vertex_buffer(renderer);
        self.update_uniform_buffer(renderer);
        self.update_shader(renderer);
        self.update_instance_buffer(renderer);
        self.changed = false;
    }

    /// Update and apply changes done to an object and returns a pipeline
    pub fn update_and_return(
        &mut self,
        renderer: &mut Renderer,
    ) -> (crate::VertexBuffers, crate::UniformBuffers, crate::Shaders) {
        let vertex_buffer = self.update_vertex_buffer_and_return(renderer);
        let uniform_buffer = self.update_uniform_buffer_and_return(renderer);
        let shader = self.update_shader_and_return(renderer);
        self.changed = false;
        (vertex_buffer, uniform_buffer, shader)
    }

    /// Update and apply changes done to the vertex buffer
    pub fn update_vertex_buffer(&mut self, renderer: &mut Renderer) {
        let updated_buffer = renderer.build_vertex_buffer(&self.vertices, &self.indices);
        self.pipeline.vertex_buffer = PipelineData::Data(updated_buffer);
    }

    /// Returns the buffer with ownership
    pub fn update_vertex_buffer_and_return(
        &mut self,
        renderer: &mut Renderer,
    ) -> crate::VertexBuffers {
        let updated_buffer = renderer.build_vertex_buffer(&self.vertices, &self.indices);
        let updated_buffer_2 = renderer.build_vertex_buffer(&self.vertices, &self.indices);
        self.pipeline.vertex_buffer = PipelineData::Data(updated_buffer);

        updated_buffer_2
    }

    /// Update and apply changes done to the shader
    pub fn update_shader(&mut self, renderer: &mut Renderer) {
        let updated_shader = renderer.build_shader(
            self.name.as_ref(),
            self.shader_builder.shader.clone(),
            Some(&self.uniform_layout),
            self.shader_settings,
        );
        self.pipeline.shader = PipelineData::Data(updated_shader);
    }

    /// Returns the buffer with ownership
    pub fn update_shader_and_return(&mut self, renderer: &mut Renderer) -> crate::Shaders {
        let updated_shader = renderer.build_shader(
            self.name.as_ref(),
            self.shader_builder.shader.clone(),
            Some(&self.uniform_layout),
            self.shader_settings,
        );
        let updated_shader2 = renderer.build_shader(
            self.name.as_ref(),
            self.shader_builder.shader.clone(),
            Some(&self.uniform_layout),
            self.shader_settings,
        );
        self.pipeline.shader = PipelineData::Data(updated_shader);

        updated_shader2
    }

    /// Update and apply changes done to the uniform buffer
    pub fn update_uniform_buffer(&mut self, renderer: &mut Renderer) {
        self.uniform_buffers[0] = renderer.build_uniform_buffer_part(
            "Transformation Matrix",
            uniform_type::Matrix::from_im(
                self.position_matrix * self.rotation_matrix * self.scale_matrix,
            ),
        );
        self.uniform_buffers[1] = renderer.build_uniform_buffer_part("Color", self.color);

        let updated_buffer = renderer.build_uniform_buffer(&self.uniform_buffers);

        self.pipeline.uniform = PipelineData::Data(Some(updated_buffer.0));
        self.uniform_layout = updated_buffer.1;
    }

    /// Returns the buffer with ownership
    pub fn update_uniform_buffer_and_return(
        &mut self,
        renderer: &mut Renderer,
    ) -> crate::UniformBuffers {
        self.uniform_buffers[0] = renderer.build_uniform_buffer_part(
            "Transformation Matrix",
            uniform_type::Matrix::from_im(
                self.position_matrix * self.rotation_matrix * self.scale_matrix,
            ),
        );
        self.uniform_buffers[1] = renderer.build_uniform_buffer_part("Color", self.color);

        let updated_buffer = renderer.build_uniform_buffer(&self.uniform_buffers);
        let updated_buffer2 = renderer.build_uniform_buffer(&self.uniform_buffers);

        self.pipeline.uniform = PipelineData::Data(Some(updated_buffer.0));
        self.uniform_layout = updated_buffer.1;

        updated_buffer2.0
    }

    /// Updates the instance buffer
    pub fn update_instance_buffer(&mut self, renderer: &mut Renderer) {
        let instance_data = self
            .instances
            .iter()
            .map(Instance::to_raw)
            .collect::<Vec<_>>();
        let instance_buffer = renderer.build_instance(instance_data);
        self.instance_buffer = instance_buffer;
    }

    /// Returns the buffer with ownership
    pub fn update_instance_buffer_and_return(&mut self, renderer: &mut Renderer) -> wgpu::Buffer {
        let instance_data = self
            .instances
            .iter()
            .map(Instance::to_raw)
            .collect::<Vec<_>>();
        let instance_buffer = renderer.build_instance(instance_data.clone());
        let instance_buffer2 = renderer.build_instance(instance_data);

        self.instance_buffer = instance_buffer;
        instance_buffer2
    }

    // ============================= FOR COPY OF PIPELINES =============================
    /// References another object's vertices
    pub fn reference_vertices(&mut self, object_id: impl StringBuffer) -> &mut Self {
        self.pipeline.vertex_buffer = PipelineData::Copy(object_id.as_string());
        self
    }

    /// References another object's shader
    pub fn reference_shader(&mut self, object_id: impl StringBuffer) -> &mut Self {
        self.pipeline.shader = PipelineData::Copy(object_id.as_string());
        self
    }

    /// References another object's texture
    pub fn reference_texture(&mut self, object_id: impl StringBuffer) -> &mut Self {
        self.pipeline.texture = PipelineData::Copy(object_id.as_string());
        self
    }

    /// References another object's uniform buffer
    pub fn reference_uniform_buffer(&mut self, object_id: impl StringBuffer) -> &mut Self {
        self.pipeline.uniform = PipelineData::Copy(object_id.as_string());
        self
    }

    // ============================= Instances =============================
    /// Add an instance to the object
    pub fn add_instance(&mut self, instance: Instance) -> &mut Self {
        self.instances.push(instance);
        self.changed = true;
        self
    }
}

/// Configuration type for ShaderBuilder
pub type ShaderConfigs = Vec<(String, Box<dyn Fn(Option<std::sync::Arc<str>>) -> String>)>;

/// Helps with building and updating shader code
pub struct ShaderBuilder {
    /// the shader itself
    pub shader: String,
    /// Should the camera effect be applied
    pub camera_effect: Option<std::sync::Arc<str>>,
    /// configurations to be applied to the shader
    pub configs: ShaderConfigs,
}

impl ShaderBuilder {
    /// Creates a new shader builder
    pub fn new(shader_source: String, camera_effect: Option<std::sync::Arc<str>>) -> Self {
        let mut shader_builder = Self {
            shader: shader_source,
            camera_effect,
            configs: vec![
                (
                    "//@CAMERA_STRUCT".to_string(),
                    Box::new(|camera_effect| {
                        if camera_effect.is_some() {
                            r#"struct CameraUniforms {
                            camera_matrix: mat4x4<f32>,
                        };
                        @group(1) @binding(0)
                        var<uniform> camera_uniform: CameraUniforms;"#
                                .to_string()
                        } else {
                            "".to_string()
                        }
                    }),
                ),
                (
                    "//@CAMERA_VERTEX".to_string(),
                    Box::new(|camera_effect| {
                        if camera_effect.is_some() {
                            r#"out.position = camera_uniform.camera_matrix * model_matrix * (transform_uniform.transform_matrix * vec4<f32>(input.position, 1.0));"#
                        .to_string()
                        } else {
                            r#"out.position = model_matrix * (transform_uniform.transform_matrix * vec4<f32>(input.position, 1.0));"#.to_string()
                        }
                    }),
                ),
            ],
        };
        shader_builder.build();

        shader_builder
    }

    /// Sets the new shader
    pub fn set_shader(&mut self, new_shader: String) {
        self.shader = new_shader;
        self.build();
    }

    /// Builds the shader with the configuration defined
    pub fn build(&mut self) {
        for i in &self.configs {
            self.shader = self.shader.replace(&i.0, &i.1(self.camera_effect.clone()));
        }
    }
}

impl Instance {
    /// Creates a new instance
    pub fn new(
        position: impl Into<Vector3>,
        rotation: impl Into<Vector3>,
        scale: impl Into<Vector3>,
    ) -> Self {
        Self {
            position: position.into(),
            rotation: rotation.into(),
            scale: scale.into(),
        }
    }

    /// Gathers all information and builds a Raw Instance to be sent to GPU
    pub fn to_raw(&self) -> InstanceRaw {
        let position_matrix = glm::translate(&DEFAULT_MATRIX_4.to_im(), &self.position.into());
        let rotation_matrix =
            nalgebra_glm::rotate(&DEFAULT_MATRIX_4.to_im(), 0f32, &self.rotation.into());
        let scale_matrix = glm::scale(&DEFAULT_MATRIX_4.to_im(), &self.scale.into());
        InstanceRaw {
            model: Matrix::from_im(position_matrix * rotation_matrix * scale_matrix),
        }
    }

    /// Sets the position
    pub fn set_position(&mut self, position: impl Into<Vector3>) {
        self.position = position.into();
    }

    /// Sets the rotation
    pub fn set_rotation(&mut self, rotation: impl Into<Vector3>) {
        self.rotation = rotation.into();
    }

    /// Sets the scale
    pub fn set_scale(&mut self, scale: impl Into<Vector3>) {
        self.scale = scale.into();
    }
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            position: Vector3::default(),
            rotation: Vector3::default(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl InstanceRaw {
    /// Instance's layout description
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We'll have to reassemble the mat4 in the shader.
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}
