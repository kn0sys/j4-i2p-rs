use j4rs::*;

pub const ROUTER_CLASS: &str = "net.i2p.router.Router";
pub const METHOD_RUN: &str = "runRouter";
pub const METHOD_IS_ALIVE: &str = "isAlive";
pub const METHOD_SHUTDOWN: &str = "shutdownGracefully";

/// Wrapper for net.i2p.router
pub struct Wrapper;

impl Wrapper {
    /// Return a new router instance.
    pub fn create_router() -> Result<Instance, errors::J4RsError> {
        log::info!("create_router");
        let jvm = JvmBuilder::new().with_base_path("opt/j4-i2p-rs").build()?;
        let router = jvm.create_instance(ROUTER_CLASS, InvocationArg::empty())?;
        Ok(router)
    }
    pub fn invoke_router(router: &Instance, method_name: &str) -> Result<(), errors::J4RsError> {
        log::info!("invoke_router::{}", method_name);
        let jvm = JvmBuilder::new().with_base_path("opt/j4-i2p-rs").build()?;
        let _ = jvm.invoke(&router, method_name, InvocationArg::empty())?;
        Ok(())
    }
}
