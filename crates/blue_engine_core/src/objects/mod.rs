//! # Objects
//! 
//! Objects make it easier to work with Blue Engine, it automates most of work needed for
//! creating 3D objects and showing them on screen. A range of default objects are available
//! as well as ability to customize each of them and even create your own! You can also
//! customize almost everything there is about them!

use crate::utils::default_resources::{DEFAULT_SHADER, DEFAULT_TEXTURE};
use crate::{
    Matrix4, Pipeline, PipelineData, Quaternion, Renderer, ShaderSettings, StringBuffer,
    TextureData, UnsignedIntType, Vector3, Vector4, Vertex,
};

mod transformation;
pub use transformation::{RotateAmount, RotateAxis};
mod instance;
pub use instance::{Instance, InstanceRaw};
mod shader_builder;
pub use shader_builder::{ShaderBuilder, ShaderConfigs};
mod resource_sharing;
mod updating;

/// Objects make it easier to work with Blue Engine, it automates most of work needed for
/// creating 3D objects and showing them on screen. A range of default objects are available
/// as well as ability to customize each of them and even create your own! You can also
/// customize almost everything there is about them!
pub struct Object {
    /// Give your object a name, which can help later on for debugging.
    pub name: std::sync::Arc<str>,
    /// A list of Vertex
    pub vertices: Vec<Vertex>,
    /// A list of indices that dictates the order that vertices appear
    pub indices: Vec<UnsignedIntType>,
    /// Describes how to uniform buffer is structures
    pub uniform_layout: wgpu::BindGroupLayout,
    /// Pipeline holds all the data that is sent to GPU, including shaders and textures
    pub pipeline: Pipeline,
    /// List of instances of this object
    pub instances: Vec<Instance>,
    /// instance buffer
    pub instance_buffer: wgpu::Buffer,
    /// Dictates the size of your object in relation to the world
    pub size: Vector3,
    /// Dictates the position of your object in pixels
    pub position: Vector3,
    /// Dictates the rotation of your object
    pub rotation: Vector3,
    // flags the object to be updated until next frame
    pub(crate) changed: bool,
    /// Transformation matrices helps to apply changes to your object, including position, orientation, ...
    /// Best choice is to let the Object system handle it
    pub translation_matrix: Matrix4,
    /// Transformation matrices helps to apply changes to your object, including position, orientation, ...
    /// Best choice is to let the Object system handle it
    pub scale_matrix: Matrix4,
    /// Transformation matrices helps to apply changes to your object, including position, orientation, ...
    /// Best choice is to let the Object system handle it
    pub rotation_quaternion: Quaternion,
    /// Transformation matrix, but inversed
    pub inverse_transformation_matrix: Matrix4,
    /// The main color of your object
    pub color: Vector4,
    /// A struct making it easier to manipulate specific parts of shader
    pub shader_builder: crate::objects::ShaderBuilder,
    /// Shader settings
    pub shader_settings: ShaderSettings,
    /// Camera have any effect on the object?
    pub camera_effect: Option<std::sync::Arc<str>>,
    /// Uniform Buffers to be sent to GPU. These are raw and not compiled for GPU yet
    pub uniform_buffers: Vec<wgpu::Buffer>,
    /// Should be rendered or not
    pub is_visible: bool,
    /// Objects with higher number get rendered later and appear "on top" when occupying the same space
    pub render_order: usize,
}
unsafe impl Send for Object {}
unsafe impl Sync for Object {}

/// Extra settings to customize objects on time of creation
#[derive(Debug, Clone)]
pub struct ObjectSettings {
    /// Should it be affected by camera?
    pub camera_effect: Option<std::sync::Arc<str>>,
    /// Shader Settings
    pub shader_settings: ShaderSettings,
}
impl Default for ObjectSettings {
    fn default() -> Self {
        Self {
            camera_effect: Some("main".into()),
            shader_settings: ShaderSettings::default(),
        }
    }
}
unsafe impl Send for ObjectSettings {}
unsafe impl Sync for ObjectSettings {}

/// A unified way to handle objects
///
/// This is a container for objects that is used to apply different operations on the objects at the same time.
/// It can deref to the object hashmap itself when needed.
pub struct ObjectStorage(std::collections::HashMap<String, Object>);
impl ObjectStorage {
    /// Creates a new object storage
    pub fn new() -> Self {
        ObjectStorage(std::collections::HashMap::new())
    }
}
impl Default for ObjectStorage {
    fn default() -> Self {
        Self::new()
    }
}
unsafe impl Send for ObjectStorage {}
unsafe impl Sync for ObjectStorage {}
crate::macros::impl_deref!(ObjectStorage, std::collections::HashMap<String, Object>);

impl Object {
    /// Creates a new object
    ///
    /// Is used to define a new object and add it to the storage. This offers full customizability
    /// and a framework for in-engine shapes to be developed.
    pub fn new(
        name: impl StringBuffer,
        vertices: Vec<Vertex>,
        indices: Vec<UnsignedIntType>,
        settings: ObjectSettings,
        renderer: &mut Renderer,
    ) -> Result<Object, crate::error::Error> {
        let vertex_buffer = renderer.build_vertex_buffer(&vertices, &indices);

        let uniform = renderer.build_uniform_buffer(&vec![
            renderer.build_uniform_buffer_part("Transformation Matrix", Matrix4::IDENTITY),
            renderer
                .build_uniform_buffer_part("Color", crate::utils::default_resources::DEFAULT_COLOR),
        ]);

        let shader_source =
            ShaderBuilder::new(DEFAULT_SHADER.to_string(), settings.camera_effect.clone());
        let shader = renderer.build_shader(
            name.as_str(),
            shader_source.shader.clone(),
            Some(&uniform.1),
            settings.shader_settings,
        );

        let texture = renderer.build_texture(
            "Default Texture",
            TextureData::Bytes(DEFAULT_TEXTURE.to_vec()),
            crate::prelude::TextureMode::Clamp,
            //crate::prelude::TextureFormat::PNG
        )?;

        let instance = Instance::default();
        let instance_buffer = renderer.build_instance(vec![instance.build()]);

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
            size: Vector3::ONE,
            position: Vector3::ZERO,
            rotation: Vector3::ZERO,
            changed: false,
            translation_matrix: Matrix4::IDENTITY,
            scale_matrix: Matrix4::IDENTITY,
            rotation_quaternion: Quaternion::IDENTITY,
            inverse_transformation_matrix: Matrix4::transpose(&Matrix4::inverse(
                &Matrix4::IDENTITY,
            )),
            color: crate::utils::default_resources::DEFAULT_COLOR,
            shader_builder: shader_source,
            shader_settings: settings.shader_settings,
            camera_effect: settings.camera_effect,
            uniform_buffers: vec![
                renderer.build_uniform_buffer_part("Transformation Matrix", Matrix4::IDENTITY),
                renderer.build_uniform_buffer_part(
                    "Color",
                    crate::utils::default_resources::DEFAULT_COLOR,
                ),
            ],
            is_visible: true,
            render_order: 0,
        })
    }
}
