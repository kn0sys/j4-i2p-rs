use j4rs::*;

pub const BYTE_ARRAY_STREAM_CLASS: &str     = "java.io.ByteArrayOutputStream";
pub const I2P_CLIENT_CLASS: &str            = "net.i2p.I2PClient";
pub const I2P_CLIENT_FACTORY_CLASS: &str    = "net.i2p.client.I2PClientFactory";
pub const I2P_DESTINATION_CLASS: &str       = "net.i2p.data.Destination";
pub const BASE64_CLASS: &str                = "net.i2p.data.Base64";
pub const METHOD_CREATE_DESTINATION: &str   = "createDestination";
pub const METHOD_ENCODE: &str               = "encode";
pub const METHOD_CREATE_CLIENT: &str        = "createClient";
pub const METHOD_TO_BASE32: &str            = "toBase32";

/// Keypair contains the secret key `sk`
///
/// and the base 32 destination address.
#[derive(Debug, Default)]
struct KeyPair {
    b32_dest: String,
    sk: String,
}

impl KeyPair {
    fn generate() -> Result<KeyPair, errors::J4RsError> {
        log::info!("Keypair::generate");
        let jvm = JvmBuilder::new().with_base_path("opt/j4-i2p-rs").build()?;
        let sk_instance = jvm.create_instance(BYTE_ARRAY_STREAM_CLASS, InvocationArg::empty())?;
        let client = jvm.invoke_static(I2P_CLIENT_FACTORY_CLASS, METHOD_CREATE_CLIENT, InvocationArg::empty())?;
        let destination = jvm.invoke(&client, METHOD_CREATE_DESTINATION, &[&InvocationArg::from(sk_instance)])?;
        let sk_bytes = jvm.invoke(&destination, "getSk", InvocationArg::empty())?;
        let b64 = jvm.invoke_static(BASE64_CLASS, METHOD_ENCODE, &[InvocationArg::from(sk_bytes)])?;
        let sk: String = jvm.to_rust(b64)?;
        let b32_dest_instance = jvm.invoke(&destination, METHOD_TO_BASE32, InvocationArg::empty())?;
        let b32_dest = jvm.to_rust(b32_dest_instance)?;
        Ok(KeyPair {b32_dest, sk})
    }
}

/// Server tunnels consist of a host, port and Pair
pub struct ServerTunnel {
    host: String,
    port: u16,
    keypair: KeyPair
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_keypair() -> Result<(), errors::J4RsError> {
        env_logger::init();
        let kp = KeyPair::generate()?;
        log::debug!("keypair: {:?}", kp);
        assert!(!kp.sk.is_empty());
        Ok(())
    }
}
