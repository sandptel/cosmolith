use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // #[error("Cli Error: {0}")]
    // FailedRegisterObject(zbus::Error),
}
