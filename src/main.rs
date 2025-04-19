// build.rs
use shaderc::Compiler;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=shaders/");
    let compiler = Compiler::new().unwrap();
    let shader_paths = ["shaders/block.vert", "shaders/block.frag"];

    fs::create_dir_all("shaders/spv").unwrap();

    for path in shader_paths.iter() {
        let source = fs::read_to_string(path).unwrap();
        let binary_result = compiler.compile_into_spirv(
            &source,
            if path.ends_with(".vert") {
                shaderc::ShaderKind::Vertex
            } else {
                shaderc::ShaderKind::Fragment
            },
            path,
            "main",
            None,
        ).unwrap();

        let spv_path = Path::new("shaders/spv")
            .join(Path::new(path).file_name().unwrap())
            .with_extension("spv");

        fs::write(spv_path, binary_result.as_binary_u8()).unwrap();
    }
}