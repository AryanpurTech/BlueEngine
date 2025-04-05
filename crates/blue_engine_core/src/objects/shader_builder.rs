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
