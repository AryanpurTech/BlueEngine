use shaderc;
use std::fs::{read_to_string, write};
use std::path::PathBuf;
use std::vec::Vec;

pub struct Shader {
    shader_paths: Vec<String>,
}

impl Shader {
    pub fn new() -> Shader {
        return Self {
            shader_paths: Vec::<String>::new(),
        };
    }

    pub fn add_shader(&mut self, shader_path: &str) {
        self.shader_paths.push(String::from(shader_path));
    }

    pub fn compile(&mut self) {
        let mut compiler = shaderc::Compiler::new().expect("Couldn't create spirv compiler");

        for i in self.shader_paths.iter() {
            let mut kind: shaderc::ShaderKind = shaderc::ShaderKind::Vertex;
            let mut supported: bool = true;
            if i.ends_with("fs") {
                kind = shaderc::ShaderKind::Fragment;
                supported = true;
            } else if i.ends_with("fs") != true && i.ends_with("vs") != true {
                supported = false;
                println!("Specified shader file isn't supported!")
            }

            if supported {
                let src =
                    read_to_string(PathBuf::from(i.to_string())).expect("Couldn't read the file");

                let output_path = PathBuf::from(i.to_string()).with_extension(format!(
                    "{}.spv",
                    PathBuf::from(i.to_string())
                        .extension()
                        .unwrap()
                        .to_str()
                        .unwrap()
                ));
                let output = compiler
                    .compile_into_spirv(&src, kind, i, "main", None)
                    .expect("Couldn't compile shader");
                println!("{:?}", output_path.to_str().unwrap());
                write(output_path, output.as_binary_u8()).expect("Couldn't write to file!");
            }
        }
    }
}
