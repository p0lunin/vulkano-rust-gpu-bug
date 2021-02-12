fn main() {
    mod cs {
        vulkano_shaders::shader! {
            ty: "compute",
            bytes: "target/spirv-builder/spirv-unknown-unknown/release/shader.spv"
        }
    }
}
