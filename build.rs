use j4rs::*;

// install i2p jars
fn main() -> Result<(), errors::J4RsError> {
    Jvm::copy_j4rs_libs_under("opt/j4-i2p-rs")?;
    Ok(())
}
