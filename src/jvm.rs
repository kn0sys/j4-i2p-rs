use j4rs::*;

/// Export the JVM from a central location for Java operations.
///
/// Set the environment variable to the `jassets` directory to
///
/// `J4I2PRS_BASE_PATH`.
pub fn new() -> Result<Jvm, errors::J4RsError> {
    let base = std::env::var("J4I2PRS_BASE_PATH")
        .unwrap_or("/tmp/opt/j4-i2p-rs".to_string());
    JvmBuilder::new().with_base_path(&base).build()
}
