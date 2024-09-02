use j4rs::*;
use crate::jvm::new as new_jvm;
use crate::error as e;

const ROUTER_CLASS: &str        = "net.i2p.router.Router";
/// Start a router instance.
pub const METHOD_RUN: &str      = "runRouter";
/// Use for checking network status. See i2p github for more methods.
pub const METHOD_IS_ALIVE: &str = "isAlive";
/// Shutdown the router.
pub const METHOD_SHUTDOWN: &str = "shutdownGracefully";
/// Pre-check router before starting tunnel operations.
const METHOD_IS_RUNNING: &str   = "isRunning";

/// Wrapper for net.i2p.router
pub struct Wrapper {
    router: Instance,
}

impl Wrapper {
    /// Return a new router instance. This function is blocking,
    ///
    /// therefore it is up to the caller to handle threading
    ///
    /// and the management of the router lifetime.
    pub fn create_router() -> Result<Self, e::J4I2PRSError> {
        log::info!("create_router");
        let jvm = new_jvm()?;
        let router = jvm.create_instance(
            ROUTER_CLASS,
            InvocationArg::empty()
        ).map_err(e::J4I2PRSError::J4rs)?;
        Ok(Wrapper {router})
    }
    /// Invoke methods on a router instance.
    ///
    /// `runRouter`, `isAlive`, and `shutDownGracefully`
    ///
    /// are the available for starting, checking status
    /// 
    /// and shutting down the router respectively.
    pub fn invoke_router(&self, method_name: &str) -> Result<(), e::J4I2PRSError> {
        log::info!("invoke_router::{}", method_name);
        let jvm = new_jvm()?;
        let _ = jvm.invoke(
            &self.router, method_name,
            InvocationArg::empty()
        ).map_err(e::J4I2PRSError::J4rs)?;
        Ok(())
    }
    /// Verify that the router is running before
    pub fn is_running(&self) -> Result<bool, e::J4I2PRSError> {
        log::info!("is_running");
        let jvm = new_jvm()?;
        let is_running = jvm.invoke(
            &self.router,
            METHOD_IS_RUNNING,
            InvocationArg::empty()
        ).map_err(e::J4I2PRSError::J4rs)?;
        let result: bool = jvm.to_rust(is_running).map_err(e::J4I2PRSError::J4rs)?;
        Ok(result)
    }
}
