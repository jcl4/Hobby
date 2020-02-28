use glsl_to_spirv::SpirvOutput;
use std::{error::Error, fs::File, io::Read, path::Path};

pub struct Pipeline {
    vs_spriv: SpirvOutput,
    fs_spriv: SpirvOutput,
}

impl Pipeline {
    pub fn new(vert_file: &Path, frag_file: &Path) -> Result<Self, Box<dyn Error>> {
        let mut vs_src = String::new();
        File::open(vert_file)?.read_to_string(&mut vs_src)?;

        let mut fs_src = String::new();
        File::open(frag_file)?.read_to_string(&mut fs_src)?;

        let vs_spriv = glsl_to_spirv::compile(&vs_src, glsl_to_spirv::ShaderType::Vertex)?;
        let fs_spriv = glsl_to_spirv::compile(&vs_src, glsl_to_spirv::ShaderType::Fragment)?;

        Ok(Pipeline { vs_spriv, fs_spriv })
    }
}
