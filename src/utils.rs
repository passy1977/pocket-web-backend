use std::{error, fmt};
use std::ffi::{CStr, CString};
use crate::bindings::{free, pocket_aes_decrypt, pocket_aes_encrypt, pocket_sha512_encrypt, pocket_t};
use crate::constants::data::EMPTY_CONFIG_JSON;

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

// pub fn byte_to_hex(bytes: &[u8]) -> String {
//     let mut hex_string = String::with_capacity(bytes.len() * 2);
//
//     for byte in bytes.iter() {
//         // Converti ogni byte in una stringa esadecimale e aggiungilo alla stringa finale.
//         hex_string.push_str(&format!("{:02X}", byte));
//     }
//
//     hex_string
// }

pub fn aes_encrypt(pocket: *mut pocket_t, str: &String) -> String {
    unsafe {
        let cstring_str = CString::new(str.as_str()).map_err(|_e| "").unwrap();
        let encrypt = pocket_aes_encrypt(pocket, cstring_str.as_ptr());
        if encrypt.is_null() {
            return String::new();
        }
        let sha512_str = CStr::from_ptr(encrypt).to_str().unwrap().to_string();

        free(encrypt.cast_mut().cast());

        sha512_str
    }
}

pub fn aes_decrypt(pocket: *mut pocket_t, str: &String) -> String {
    unsafe {
        let cstring_str = CString::new(str.as_str()).map_err(|_e| "").unwrap();
        let decrypt = pocket_aes_decrypt(pocket, cstring_str.as_ptr());
        if decrypt.is_null() {
            return String::new();
        }
        let sha512_str = CStr::from_ptr(decrypt).to_str().unwrap().to_string();

        free(decrypt.cast_mut().cast());

        sha512_str
    }
}

pub fn sha512_encrypt(str: &String) -> String {
    unsafe {
        let cstring_str = CString::new(str.as_str()).map_err(|_e| "").unwrap();
        let encrypt = pocket_sha512_encrypt(cstring_str.as_ptr());
        if encrypt.is_null() {
            return String::new();
        }
        let sha512_str = CStr::from_ptr(encrypt).to_str().unwrap().to_string();


        free(encrypt.cast_mut().cast());


        sha512_str
    }
}