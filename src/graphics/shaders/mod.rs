use shaderc;

pub fn shader_compile(kind: shaderc::ShaderKind, source: &str) -> Vec<u8> {
    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.add_macro_definition("EP", Some("main"));
    compiler
        .compile_into_spirv(
            source,
            kind, // shaderc::ShaderKind::Vertex,
            "shader.glsl",
            "main",
            Some(&options),
        )
        .unwrap()
        .as_binary_u8()
        .into()
}

static VERT: &'static str = include_str!("quad.vert");

pub fn vs_compile() -> Vec<u8> {
    shader_compile(shaderc::ShaderKind::Vertex, VERT)
}

static FRAG: &'static str = include_str!("quad.frag");

pub fn fs_compile() -> Vec<u8> {
    shader_compile(shaderc::ShaderKind::Fragment, FRAG)
}
