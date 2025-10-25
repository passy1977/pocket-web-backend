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

static mut PRINT_ONCE: bool = false;

pub(crate) fn configure_cors() -> actix_cors::Cors {
    use crate::constants::conf::{CORS_MAX_AGE, CORS_ALLOWED_METHODS, CORS_ALLOWED_HEADERS};
    use actix_cors::Cors;
    use std::env;

    let mut allowed_origins = Vec::<String>::new();
    #[cfg(debug_assertions)]
    {
        allowed_origins.push("http://localhost:8080".to_string());
        allowed_origins.push("http://127.0.0.1:8080".to_string());
    }

    if let Ok(additional_origins) = env::var("CORS_ALLOWED_ORIGINS") {
        for origin in additional_origins.split(',') {
            let origin = origin.trim();
            if !origin.is_empty() {
                allowed_origins.push(origin.to_string());
            }
        }
    }

    let mut cors = Cors::default();

    if !allowed_origins.is_empty() {
        cors = cors.allowed_methods(CORS_ALLOWED_METHODS.to_vec())
            .allowed_headers(CORS_ALLOWED_HEADERS.to_vec())
            .max_age(CORS_MAX_AGE);

        for origin in &allowed_origins {
            cors = cors.allowed_origin(origin.as_str());
            unsafe {
                if !PRINT_ONCE {
                    println!("CORS allowed origin: {}", origin);
                    PRINT_ONCE = true;
                }
            }

        }
    } else {
        cors = Cors::permissive();
        unsafe {
            if !PRINT_ONCE {
                eprintln!("\x1b[93mWARNING: No CORS_ALLOWED_ORIGINS specified. CORS is set to permissive mode.\x1b[0m");
                PRINT_ONCE = true;
            }
        }

    }

    cors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_sets_equal_same_elements() {
        let a = vec![1, 2, 3, 4];
        let b = vec![4, 3, 2, 1]; // Same elements, different order
        assert!(are_sets_equal(&a, &b));
    }

    #[test]
    fn test_are_sets_equal_different_elements() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 4];
        assert!(!are_sets_equal(&a, &b));
    }

    #[test]
    fn test_are_sets_equal_empty_sets() {
        let a: Vec<i32> = vec![];
        let b: Vec<i32> = vec![];
        assert!(are_sets_equal(&a, &b));
    }

    #[test]
    fn test_are_sets_equal_duplicates() {
        let a = vec![1, 2, 2, 3];
        let b = vec![1, 2, 3];
        assert!(are_sets_equal(&a, &b)); // Duplicates should be ignored
    }

    #[test]
    fn test_are_sets_equal_strings() {
        let a = vec!["hello", "world"];
        let b = vec!["world", "hello"];
        assert!(are_sets_equal(&a, &b));
    }

    #[test]
    fn test_error_display() {
        let error = Error::Undefine;
        assert_eq!(format!("{}", error), "Undefine");

        let error = Error::Msg("Custom error message".to_string());
        assert_eq!(format!("{}", error), "Custom error message");
    }

    #[test]
    fn test_error_description() {
        
        let error = crate::utils::Error::Undefine;
        assert_eq!(format!("{}", error), "Undefine");

        let error = crate::utils::Error::Msg("Custom message".to_string());
        assert_eq!(format!("{}", error), "Custom message");
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(Error::Undefine, Error::Undefine);
        assert_eq!(Error::Msg("test".to_string()), Error::Msg("test".to_string()));
        assert_ne!(Error::Undefine, Error::Msg("test".to_string()));
    }

    // Test for configure_cors - verify that CORS configuration is valid
    #[test]
    fn test_configure_cors_basic() {
        let _cors = configure_cors();

        // We can't easily test the internal content of Cors,
        // but we can verify that the function doesn't panic
        // and returns a valid Cors object
    }
}
