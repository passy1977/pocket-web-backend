use std::{error, fmt};
use std::ffi::{CStr, CString};
use std::hash::Hash;
use crate::bindings::{free, pocket_aes_decrypt, pocket_aes_encrypt, pocket_sha512_encrypt, pocket_t};

pub(crate) type Result<T, E = &'static str> = std::result::Result<T, E>;

#[allow(dead_code)]
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
        let c_str = CString::new(str.to_string()).expect("CString::new failed");
        let result_ptr = pocket_aes_encrypt(pocket, c_str.as_ptr());

        if result_ptr.is_null() {
            panic!("pocket_aes_encrypt returned null");
        }

        let c_str = CStr::from_ptr(result_ptr);

        let ret = c_str.to_string_lossy().into_owned().clone();

        free(result_ptr.cast_mut().cast());

        ret
    }
}

pub fn aes_decrypt(pocket: *mut pocket_t, encrypted: &String) -> String {
    unsafe {
        let c_encrypted = CString::new(encrypted.to_string()).expect("CString::new failed");
        let result_ptr = pocket_aes_decrypt(pocket, c_encrypted.as_ptr());

        if result_ptr.is_null() {
            panic!("pocket_aes_encrypt returned null");
        }

        let c_str = CStr::from_ptr(result_ptr);

        let ret = c_str.to_string_lossy().into_owned().clone();

        free(result_ptr.cast_mut().cast());

        ret
    }
}

pub fn sha512_encrypt(str: &String) -> String {
    unsafe {
        let c_str = CString::new(str.as_str()).map_err(|_e| "").unwrap();
        let result_ptr = pocket_sha512_encrypt(c_str.as_ptr());

        if result_ptr.is_null() {
            panic!("pocket_aes_encrypt returned null");
        }

        let c_str = CStr::from_ptr(result_ptr);

        let ret = c_str.to_string_lossy().into_owned().clone();

        free(result_ptr.cast_mut().cast());

        ret
    }
}

pub fn are_sets_equal<T>(a: &[T], b: &[T]) -> bool
where T: PartialEq + Eq + Hash + Clone
{
    use std::collections::HashSet;

    let set_a: HashSet<_> = a.iter().cloned().collect();
    let set_b: HashSet<_> = b.iter().cloned().collect();

    set_a == set_b
}

// pub fn hex_to_bytes(hex_str: &str) -> Result<Vec<u8>, String> {
//     if hex_str.len() % 2 != 0 {
//         return Err("Hex string has an odd length".to_string());
//     }

//     let mut bytes = Vec::with_capacity(hex_str.len() / 2);
    
//     for chunk in hex_str.as_bytes().chunks(2) {
//         let num = u8::from_str_radix(std::str::from_utf8(chunk).map_err(|_| "Invalid UTF-8 sequence")?, 16)
//             .map_err(|_| "Failed to parse hex digit")?;
//         bytes.push(num);
//     }

//     Ok(bytes)
// }

// pub fn bytes_to_hex(bytes: &[u8]) -> String {
//     let hex_str = bytes.iter()
//         .map(|byte| format!("{:02x}", byte))
//         .collect::<String>();
    
//     hex_str
// }
