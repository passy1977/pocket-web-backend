use std::{error, fmt};


pub(crate) type Result<T, E = &'static str> = std::result::Result<T, E>;

#[derive(Debug, PartialEq, Eq)]
pub(crate)  enum Error {
    Undefine,
    Msg(String)
}

impl fmt::Display for Error {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use crate::utils::Error::*;
        
        match self {
            Undefine => write!(f, "Undefine"),
            Msg(msg) => write!(f, "{}", msg)
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use crate::utils::Error::*;
        
        match self {
            Undefine => "UndefineError",
            Msg(msg) => msg,
            //_ => "GenericError"
        }
    }
}

pub fn byte_to_hex(bytes: &[u8]) -> String {
    let mut hex_string = String::with_capacity(bytes.len() * 2);

    for byte in bytes.iter() {
        // Converti ogni byte in una stringa esadecimale e aggiungilo alla stringa finale.
        hex_string.push_str(&format!("{:02X}", byte));
    }
    
    hex_string
}