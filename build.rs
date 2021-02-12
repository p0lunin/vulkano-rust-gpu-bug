use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    spirv_builder::SpirvBuilder::new("shader")
        .spirv_version(1, 0)
        .print_metadata(true)
        .build()?;

    Ok(())
}