use j4rs::*;

// install i2p jars
fn main() -> Result<(), errors::J4RsError> {
    let base = std::env::var("J4I2PRS_BASE_PATH")
        .unwrap_or("/tmp/opt/j4-i2p-rs".to_string());
    Jvm::copy_j4rs_libs_under(&base)?;
    Ok(())
}
