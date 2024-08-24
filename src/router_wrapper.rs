use j4rs::*;

const BASE_PATH: &str           = "opt.j4-i2p-rs";
const ROUTER_CLASS: &str        = "net.i2p.router.Router";
pub const METHOD_RUN: &str      = "runRouter";
pub const METHOD_IS_ALIVE: &str = "isAlive";
pub const METHOD_SHUTDOWN: &str = "shutdownGracefully";

/// Wrapper for net.i2p.router
pub struct Wrapper {
    router: Instance,
}

impl Wrapper {
    /// Return a new router instance.
    pub fn create_router() -> Result<Self, errors::J4RsError> {
        log::info!("create_router");
        let jvm = JvmBuilder::new().with_base_path(BASE_PATH).build()?;
        let router = jvm.create_instance(ROUTER_CLASS, InvocationArg::empty())?;
        Ok(Wrapper {router})
    }
    pub fn invoke_router(&self, method_name: &str) -> Result<(), errors::J4RsError> {
        log::info!("invoke_router::{}", method_name);
        let jvm = JvmBuilder::new().with_base_path(BASE_PATH).build()?;
        let _ = jvm.invoke(&self.router, method_name, InvocationArg::empty())?;
        Ok(())
    }
}
