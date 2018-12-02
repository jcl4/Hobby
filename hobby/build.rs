extern crate shaderc;

use std::error::Error;
use std::fs;

use shaderc::ShaderKind;

fn main() -> Result<(), Box<Error>> {
    // println!("cargo:rerun-if-changed=source_assets/shaders/");
    let mut compiler = shaderc::Compiler::new().unwrap();

    for entry in fs::read_dir("source_assets/shaders/")? {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            let input_path = entry.path();

            let shader_type =
                input_path
                    .extension()
                    .and_then(|ext| match ext.to_string_lossy().as_ref() {
                        "vert" => Some(ShaderKind::Vertex),
                        "frag" => Some(ShaderKind::Fragment),
                        _ => None,
                    });

            if let Some(shader_type) = shader_type {
                let source = fs::read_to_string(&input_path)?;
                let file_name = input_path.file_name().unwrap().to_string_lossy();
                let compiled_source =
                    compiler.compile_into_spirv(&source, shader_type, &file_name, "main", None)?;

                let output_path = format!("assets/shaders/{}.spv", file_name);

                fs::write(output_path, compiled_source.as_binary_u8())?;
            }
        }
    }

    Ok(())
}
