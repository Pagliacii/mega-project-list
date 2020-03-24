use std::error::Error;
use std::fmt;
use std::io;
use std::process;
use std::str;

#[derive(Debug)]
pub enum WCErrorKind {
    InvalidBytes(str::Utf8Error),
    NotAllowed,
    OpenFailed(io::Error),
    Other,
    ReadFailed(io::Error),
}

#[derive(Debug)]
pub struct WCError {
    code: i32,
    kind: WCErrorKind,
    message: String,
}

impl WCError {
    pub fn new(code: i32, kind: WCErrorKind, message: &str) -> Self {
        Self {
            code,
            kind,
            message: String::from(message),
        }
    }

    pub fn exit(&self) -> ! {
        eprintln!("wc: {}", self);
        if let Some(e) = self.source() {
            eprintln!("caused by: {}", e);
        }
        process::exit(self.code);
    }
}

impl fmt::Display for WCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for WCError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            WCErrorKind::InvalidBytes(e) => Some(e),
            WCErrorKind::NotAllowed => None,
            WCErrorKind::OpenFailed(e) => Some(e),
            WCErrorKind::ReadFailed(e) => Some(e),
            WCErrorKind::Other => None,
        }
    }
}
