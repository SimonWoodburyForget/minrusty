use shaderc;

static VERT: &'static str = include_str!("quad.vert");

pub fn vs_compile() -> Vec<u8> {
    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.add_macro_definition("EP", Some("main"));
    compiler
        .compile_into_spirv(
            VERT,
            shaderc::ShaderKind::Vertex,
            "shader.glsl",
            "main",
            Some(&options),
        )
        .unwrap()
        .as_binary_u8()
        .into()
}

static FRAG: &'static str = include_str!("quad.frag");

pub fn fs_compile() -> Vec<u8> {
    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.add_macro_definition("EP", Some("main"));
    compiler
        .compile_into_spirv(
            FRAG,
            shaderc::ShaderKind::Fragment,
            "shader.glsl",
            "main",
            Some(&options),
        )
        .unwrap()
        .as_binary_u8()
        .into()
}
