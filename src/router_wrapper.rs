use j4rs::*;

const BASE_PATH: &str           = "opt/j4-i2p-rs";
const ROUTER_CLASS: &str        = "net.i2p.router.Router";
/// Start a router instance.
pub const METHOD_RUN: &str      = "runRouter";
/// Use for checking network status. See i2p github for more methods.
pub const METHOD_IS_ALIVE: &str = "isAlive";
/// Shutdown the router.
pub const METHOD_SHUTDOWN: &str = "shutdownGracefully";
/// Pre-check router before starting tunnel operations.
const METHOD_IS_RUNNING: &str = "isRunning";

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
    pub fn create_router() -> Result<Self, errors::J4RsError> {
        log::info!("create_router");
        let jvm = JvmBuilder::new().with_base_path(BASE_PATH).build()?;
        let router = jvm.create_instance(ROUTER_CLASS, InvocationArg::empty())?;
        Ok(Wrapper {router})
    }
    /// Invoke methods on a router instance.
    ///
    /// `runRouter`, `isAlive`, and `shutDownGracefully`
    ///
    /// are the available for starting, checking status
    /// 
    /// and shutting down the router respectively.
    pub fn invoke_router(&self, method_name: &str) -> Result<(), errors::J4RsError> {
        log::info!("invoke_router::{}", method_name);
        let jvm = JvmBuilder::new().with_base_path(BASE_PATH).build()?;
        let _ = jvm.invoke(&self.router, method_name, InvocationArg::empty())?;
        Ok(())
    }
    /// Verify that the router is running before
    pub fn is_running(&self) -> Result<bool, errors::J4RsError> {
        log::info!("is_running");
        let jvm = JvmBuilder::new().with_base_path(BASE_PATH).build()?;
        let is_running = jvm.invoke(&self.router, METHOD_IS_RUNNING, InvocationArg::empty())?;
        let result: bool = jvm.to_rust(is_running)?;
        Ok(result)
    }
}
