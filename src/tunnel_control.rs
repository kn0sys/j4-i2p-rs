use j4rs::*;
use rand::RngCore;
use crate::jvm::new as new_jvm;
use crate::error as e;

const I2P_TUNNEL_CLASS: &str            = "net.i2p.i2ptunnel.I2PTunnel";
const BYTE_ARRAY_STREAM_CLASS: &str     = "java.io.ByteArrayOutputStream";
const FILE_OUTPUT_STREAM_CLASS: &str    = "java.io.FileOutputStream";
const I2P_CLIENT_FACTORY_CLASS: &str    = "net.i2p.client.I2PClientFactory";
const BASE64_CLASS: &str                = "net.i2p.data.Base64";
const METHOD_CREATE_DESTINATION: &str   = "createDestination";
const METHOD_ENCODE: &str               = "encode";
const METHOD_CREATE_CLIENT: &str        = "createClient";
const METHOD_TO_BASE32: &str            = "toBase32";
const METHOD_GET_SK: &str               = "getSk";
const METHOD_CLOSE: &str                = "close";
const METHOD_WRITE: &str                = "write";
const METHOD_DECODE: &str               = "decode";

/// Keypair contains the secret key `sk`
///
/// and the base 32 destination address.
#[derive(Debug, Default)]
pub struct KeyPair {
    b32_dest: String,
    sk: String,
}

impl KeyPair {
    /// Generates a new KeyPair. Necessary for creating a 
    ///
    /// server tunnel.
    fn generate() -> Result<KeyPair, e::J4I2PRSError> {
        log::info!("Keypair::generate");
        let jvm = new_jvm()?;
        let sk_instance = jvm.create_instance(BYTE_ARRAY_STREAM_CLASS, InvocationArg::empty())
            .map_err(e::J4I2PRSError::J4rs)?;
        let client = jvm.invoke_static(I2P_CLIENT_FACTORY_CLASS, METHOD_CREATE_CLIENT, InvocationArg::empty())
            .map_err(e::J4I2PRSError::J4rs)?;
        let destination = jvm.invoke(&client, METHOD_CREATE_DESTINATION, &[&InvocationArg::from(sk_instance)])
            .map_err(e::J4I2PRSError::J4rs)?;
        let sk_bytes = jvm.invoke(&destination, METHOD_GET_SK, InvocationArg::empty())
            .map_err(e::J4I2PRSError::J4rs)?;
        let b64 = jvm.invoke_static(BASE64_CLASS, METHOD_ENCODE, &[InvocationArg::from(sk_bytes)])
            .map_err(e::J4I2PRSError::J4rs)?;
        let sk: String = jvm.to_rust(b64).map_err(e::J4I2PRSError::J4rs)?;
        let b32_dest_instance = jvm.invoke(&destination, METHOD_TO_BASE32, InvocationArg::empty())
            .map_err(e::J4I2PRSError::J4rs)?;
        let b32_dest: String = jvm.to_rust(b32_dest_instance)
            .map_err(e::J4I2PRSError::J4rs)?;
        Ok(KeyPair {b32_dest, sk})
    }
}

#[derive(Debug)]
/// Tunnel Types.
///
/// `Http` - http proxy for inbound/outbound proxied data
///
/// `Server` - web application tunnels
///
/// `Socks` - socks proxy tunnel
pub enum TunnelType {
    Http,
    Server,
    Socks,
}

impl TunnelType {
    pub fn value(&self) -> String {
        match *self {
            TunnelType::Http => String::from("http"),
            TunnelType::Server => String::from("server"),
            TunnelType::Socks => String::from("socks"),
        }
    }
}

#[derive(Debug)]
/// Tunnels consist of a host, port, KeyPair (server only) and type
pub struct Tunnel {
    host: String,
    keypair: KeyPair,
    port: u16,
    tunnel_type: TunnelType,
}

impl Default for Tunnel {
    fn default() -> Self {
        Tunnel {
            host: Default::default(),
            keypair: Default::default(),
            port: Default::default(),
            tunnel_type: TunnelType::Server,
        }
    }
}

impl Tunnel {
    /// Create a tunnel.
    pub fn new(host: String, port: u16, tunnel_type: TunnelType) -> Result<Self, e::J4I2PRSError> {
        let mut keypair: KeyPair = Default::default();
        match tunnel_type {
            TunnelType::Server => {
                keypair = KeyPair::generate()?;
                Ok(Tunnel { host, keypair, port, tunnel_type, })
            },
            _ => Ok(Tunnel { host, keypair, port, tunnel_type, })
        }
    }
    /// Start the associated tunnel based on type
    pub fn start(&self) -> Result<(), e::J4I2PRSError> {
        match self.tunnel_type {
            TunnelType::Http => self.start_http(),
            TunnelType::Server => self.start_server(),
            TunnelType::Socks => self.start_socks(),
        }
    }
    /// Start a server tunnel.
    fn start_server(&self) -> Result<(), e::J4I2PRSError> {
        log::info!("starting {} tunnel on {}", self.tunnel_type.value(), self.keypair.b32_dest);
        let jvm = new_jvm()?;
        let mut data = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut data);
        let uuid = hex::encode(data);
        let sk_path = format!("sk.{}.dat", uuid);
        let b64_decode = jvm.invoke_static(
            BASE64_CLASS, METHOD_DECODE,
            &[InvocationArg::try_from(self.keypair.sk.clone()).map_err(e::J4I2PRSError::J4rs)?]
        ).map_err(e::J4I2PRSError::J4rs)?;
        let file_output_stream = jvm.create_instance(
            FILE_OUTPUT_STREAM_CLASS,
            &[InvocationArg::try_from(&sk_path).map_err(e::J4I2PRSError::J4rs)?]
        ).map_err(e::J4I2PRSError::J4rs)?;
        let _ = jvm.invoke(
            &file_output_stream,
            METHOD_WRITE,
            &[InvocationArg::from(b64_decode)]
        ).map_err(e::J4I2PRSError::J4rs)?;
        let _ = jvm.invoke(
            &file_output_stream,
            METHOD_CLOSE,
            InvocationArg::empty()).map_err(e::J4I2PRSError::J4rs)?;
        let cwd_path = std::env::current_dir().map_err(e::J4I2PRSError::StdIo)?;
        let cwd = cwd_path.to_str().unwrap_or_default();
        let array = jvm.create_java_array("java.lang.String", &[
            InvocationArg::try_from("-die").map_err(e::J4I2PRSError::J4rs)?,
            InvocationArg::try_from("-nocli").map_err(e::J4I2PRSError::J4rs)?,
            InvocationArg::try_from("-e").map_err(e::J4I2PRSError::J4rs)?,
            InvocationArg::try_from(["server", &format!("{} {} {}/{}", self.host, self.port, cwd, &sk_path)].join(" ")).map_err(e::J4I2PRSError::J4rs)?
        ]).map_err(e::J4I2PRSError::J4rs)?;
        let _ = jvm.create_instance(I2P_TUNNEL_CLASS, &[InvocationArg::from(array)]).map_err(e::J4I2PRSError::J4rs)?;
        Ok(())
    }
    /// Start the I2P HTTP Proxy.
    fn start_http(&self) -> Result<(), e::J4I2PRSError> {
        log::info!("starting {} proxy tunnel on port {}", self.tunnel_type.value(), self.port);
        let jvm = new_jvm()?;
        let array = jvm.create_java_array("java.lang.String", &[
            InvocationArg::try_from("-die").map_err(e::J4I2PRSError::J4rs)?,
            InvocationArg::try_from("-nocli").map_err(e::J4I2PRSError::J4rs)?,
            InvocationArg::try_from("-e").map_err(e::J4I2PRSError::J4rs)?,
            InvocationArg::try_from("config localhost 7654").map_err(e::J4I2PRSError::J4rs)?,
            InvocationArg::try_from("-e").map_err(e::J4I2PRSError::J4rs)?,
            InvocationArg::try_from(["httpclient", &format!("{}", self.port)].join(" ")).map_err(e::J4I2PRSError::J4rs)?
        ]).map_err(e::J4I2PRSError::J4rs)?;
        let _ = jvm.create_instance(I2P_TUNNEL_CLASS, &[InvocationArg::from(array)]).map_err(e::J4I2PRSError::J4rs)?;
        Ok(())
    }
    /// Start the SOCKS proxy.
    fn start_socks(&self) -> Result<(), e::J4I2PRSError> {
        log::info!("starting {} proxy tunnel on port {}", self.tunnel_type.value(), self.port);
        let jvm = new_jvm()?;
        let array = jvm.create_java_array("java.lang.String", &[
            InvocationArg::try_from("-die").map_err(e::J4I2PRSError::J4rs)?,
            InvocationArg::try_from("-nocli").map_err(e::J4I2PRSError::J4rs)?,
            InvocationArg::try_from("-e").map_err(e::J4I2PRSError::J4rs)?,
            InvocationArg::try_from(["sockstunnel", &format!("{}", self.port)].join(" ")).map_err(e::J4I2PRSError::J4rs)?
        ]).map_err(e::J4I2PRSError::J4rs)?;
        let _ = jvm.create_instance(I2P_TUNNEL_CLASS, &[InvocationArg::from(array)]).map_err(e::J4I2PRSError::J4rs)?;
        Ok(())
    }
    /// Get the Base 32 destination of the server tunnel.
    pub fn get_destination(&self) -> String {
        String::from(&self.keypair.b32_dest)
    }
    /// Get the port for the tunnel.
    pub fn get_port(&self) -> u16 {
        self.port
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_keypair() -> Result<(), e::J4I2PRSError> {
        let kp = KeyPair::generate()?;
        assert!(kp.b32_dest.contains(".b32.i2p"));
        Ok(())
    }
}
