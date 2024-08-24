use j4rs::*;
use rand::RngCore;

const BASE_PATH: &str                   = "opt/j4-i2p-rs";
const I2P_TUNNEL_CLASS: &str            = "net.i2p.i2ptunnel";
const BYTE_ARRAY_STREAM_CLASS: &str     = "java.io.ByteArrayOutputStream";
const PATH_CLASS: &str                  = "java.nio.file.PATH";
const FILES_CLASS: &str                 = "java.nio.file.Files";
const FILE_CLASS: &str                  = "java.io.File";
const I2P_CLIENT_CLASS: &str            = "net.i2p.I2PClient";
const I2P_CLIENT_FACTORY_CLASS: &str    = "net.i2p.client.I2PClientFactory";
const I2P_DESTINATION_CLASS: &str       = "net.i2p.data.Destination";
const BASE64_CLASS: &str                = "net.i2p.data.Base64";
const METHOD_CREATE_DESTINATION: &str   = "createDestination";
const METHOD_ENCODE: &str               = "encode";
const METHOD_CREATE_CLIENT: &str        = "createClient";
const METHOD_TO_BASE32: &str            = "toBase32";
const METHOD_GET_SK: &str               = "getSk";
const METHOD_OF: &str                   = "of";
const METHOD_WRITE: &str                = "write";
const METHOD_DELETE_ON_EXIT: &str       = "deleteOnExit";
const METHOD_DECODE: &str               = "decode";

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
        let jvm = JvmBuilder::new().with_base_path(BASE_PATH).build()?;
        let sk_instance = jvm.create_instance(BYTE_ARRAY_STREAM_CLASS, InvocationArg::empty())?;
        let client = jvm.invoke_static(I2P_CLIENT_FACTORY_CLASS, METHOD_CREATE_CLIENT, InvocationArg::empty())?;
        let destination = jvm.invoke(&client, METHOD_CREATE_DESTINATION, &[&InvocationArg::from(sk_instance)])?;
        let sk_bytes = jvm.invoke(&destination, METHOD_GET_SK, InvocationArg::empty())?;
        let b64 = jvm.invoke_static(BASE64_CLASS, METHOD_ENCODE, &[InvocationArg::from(sk_bytes)])?;
        let sk: String = jvm.to_rust(b64)?;
        let b32_dest_instance = jvm.invoke(&destination, METHOD_TO_BASE32, InvocationArg::empty())?;
        let b32_dest: String = jvm.to_rust(b32_dest_instance)?;
        Ok(KeyPair {b32_dest, sk})
    }
}

/// Server tunnels consist of a host, port and KeyPair
pub struct ServerTunnel {
    host: String,
    keypair: KeyPair,
    port: u16,
}

impl ServerTunnel {
    fn new(keypair: KeyPair, host: String, port: u16) -> Self {
        ServerTunnel {
            host,
            keypair,
            port
        }
    }
    fn start(&self) -> Result<(), errors::J4RsError> {
        log::info!("starting tunnel on {}", self.keypair.b32_dest);
        let jvm = JvmBuilder::new().with_base_path(BASE_PATH).build()?;
        let mut data = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut data);
        let uuid = hex::encode(data);
        let sk_path = format!("sk.{}.dat", uuid);
        let path = jvm.invoke_static(PATH_CLASS, METHOD_OF, &[InvocationArg::try_from(&sk_path)?])?;
        let b64_decode = jvm.invoke_static(BASE64_CLASS, METHOD_DECODE, &[InvocationArg::try_from(self.keypair.b32_dest.clone())?])?;
        let _ = jvm.invoke_static(FILES_CLASS, METHOD_WRITE, &[InvocationArg::from(path), InvocationArg::from(b64_decode)])?;
        let file = jvm.create_instance(FILE_CLASS, &[InvocationArg::try_from(&sk_path)?])?;
        let _ = jvm.invoke(&file, METHOD_DELETE_ON_EXIT, InvocationArg::empty())?;
        let server_arg = format!("server {} {} {}", self.host, self.port, &sk_path);
        let args: &[String] = &["-die".to_string(), "-nocli".to_string(), "-e".to_string(), server_arg];
        let _ = jvm.create_instance(I2P_TUNNEL_CLASS, &[InvocationArg::try_from(args)?])?;
        Ok(())
    } 
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
