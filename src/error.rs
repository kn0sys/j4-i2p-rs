use thiserror::Error;
use j4rs::errors::J4RsError;

#[derive(Debug, Error)]
    #[error("j4rs error. See wrapper.log for more info.")]
    pub enum J4I2PRSError {
        J4rs(J4RsError),
        StdIo(std::io::Error),
    }

