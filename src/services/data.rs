use crate::constants::data::*;
use crate::utils::Result;
use std::path::{Path, PathBuf};
use std::{env, fs};
use crate::services::cli::Cli;

pub struct Data {
    
    pub file_data_path: PathBuf,
    
    /// Ip where WebSocket listen
    pub ip: String,

    /// Connection port
    pub port: u16,

    ///Path and filename where find ssl cert
    pub ssl_cert: String,

    ///Path and filename where find key cert
    pub ssl_key: String,
    
    pub update : bool
    
}



impl Data {


    pub fn init() -> Result<Self> {
       

        let dir_path = match env::var("HOME")  {
            Ok(home) => {
                let mut ret = PathBuf::from(home);
                ret.push(Path::new(DATA_FOLDER));
                ret
            },
            Err(_) => return Err("Could not find home directory.".into())
        };

        

        if !dir_path.exists() {
            match fs::create_dir_all(&dir_path) {
                Ok(_) => (),
                Err(e) => return Err("Impossible to create dir: {e}".into())
            }
        }

        
        let mut file_data_path = dir_path.join(DATA_FILE);
        
        if !file_data_path.exists() {
            match fs::File::create(&file_data_path) {
                Ok(_) => (),
                Err(e) => return Err("Error in creation file: {e}"),
            }
        }

        let mut ret = Self {
            file_data_path,
            ip: "".to_string(),
            port: 0,
            ssl_cert: "".to_string(),
            ssl_key: "".to_string(),
            update: false
        };
        
        ret.load()?;

        Cli::update(&mut ret);

        if ret.update {
            ret.store()?;
        }
        
        Ok(ret)
    }
    
    pub fn load(&mut self) -> Result<()> {

        Ok(())
    }
    pub fn store(&mut self) -> Result<()> {

        Ok(())
    }
}