use j4rs::*;
use crate::error as e;
/// Export the JVM from a central location for Java operations.
///
/// Set the environment variable to the `jassets` directory to
///
/// `J4I2PRS_BASE_PATH`.
pub fn new() -> Result<Jvm, e::J4I2PRSError> {
    let base = std::env::var("J4I2PRS_BASE_PATH")
        .unwrap_or("opt/j4-i2p-rs".to_string());
    JvmBuilder::new().with_base_path(&base).build().map_err(e::J4I2PRSError::J4rs)
}
