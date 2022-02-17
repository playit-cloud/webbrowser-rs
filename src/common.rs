use std::ffi::OsStr;
use crate::{Error, ErrorKind, Result};
use std::process::{Command, ExitStatus, Stdio};

/// Analyses return code from a command ExitStatus to create the right
/// Result<()>
pub fn from_status(res: Result<ExitStatus>) -> Result<()> {
    match res {
        Ok(status) => {
            if let Some(code) = status.code() {
                if code == 0 {
                    Ok(())
                } else {
                    Err(Error::new(
                        ErrorKind::Other,
                        format!("return code {}", code),
                    ))
                }
            } else {
                Err(Error::new(ErrorKind::Other, "interrupted by signal"))
            }
        }
        Err(err) => Err(err),
    }
}

pub fn build_command<S: AsRef<OsStr>>(program: S) -> Command {
    let mut command = Command::new(program);

    #[cfg(feature = "suppress-logs")]
    command
        .stderr(Stdio::null())
        .stdout(Stdio::null());

    command
}