use crate::constants::{data::*, conf::*};
use crate::utils::{sha512_encrypt, Result};
use crate::services::cli::Cli;
use std::path::{Path, PathBuf};
use std::{env, fs};
use std::fs::File;
use std::io::{self, Error, ErrorKind, Read, Write};
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Data {

    #[serde(skip_serializing, skip_deserializing)]
    file_data_path: PathBuf,

    #[serde(skip_serializing, skip_deserializing)]
    pub dir_path: PathBuf,

    /// IP address on which the server listens
    pub ip: String,

    /// Connection port
    pub port: u16,

    #[serde(skip_serializing, skip_deserializing)]
    pub(super) update : bool,

    pub max_threads: usize,

    /// Session expiration time in seconds
    pub session_expiration_time: u32
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
                Err(_e) => return Err("Impossible to create dir: {e}".into())
            }
        }

        
        let file_data_path = dir_path.join(DATA_FILE);
        
        if !file_data_path.exists() {
            match fs::File::create(&file_data_path) {
                Ok(_) => (),
                Err(_e) => return Err("Error in creation file: {e}"),
            }
        }

        let mut ret = Self {
            file_data_path,
            dir_path,
            ip: IP.to_string(),
            port: PORT,
            update: false,
            max_threads: MAX_BLOCKING_THREADS,
            session_expiration_time: SESSION_EXPIRATION_TIME
        };
        
        if let Err(e) = ret.load() {
            ret.update = true;
            eprintln!("Error loading data: {e}");
        }

        Cli::update(&mut ret);

        if ret.update {
            if let Err(_e) = ret.store() {
                return Err("Error updating file: {e}");
            }
        }
        
        Ok(ret)
    }
    
    pub fn load(&mut self) -> Result<(), io::Error> {

        let mut file = File::open(self.file_data_path.as_path())?;

        let mut data = String::new();

        file.read_to_string(&mut data)?;

        let data : Data = serde_json::from_str(&data)?;

        self.ip = data.ip;
        self.port = data.port;
        self.update = false;
        self.max_threads = data.max_threads;
        self.session_expiration_time = data.session_expiration_time;

        Ok(())
    }
    pub fn store(&self) -> Result<(), io::Error> {

        let data_json = serde_json::to_string(&self)?;

        let mut file = File::create(self.file_data_path.clone())?;

        file.write_all(data_json.as_bytes())?;

        Ok(())
    }
    
    pub fn load_config_json(&self, email: &String) -> Result<String, Error> {

        if !self.dir_path.exists() {
            return Err(Error::new(ErrorKind::NotFound, "Dir not exist {dir_path}"))
        }

        let email = sha512_encrypt(email);

        let mut config_json_file = self.dir_path.clone();
        config_json_file.push(email);
        config_json_file.set_extension("json");
        
        if !config_json_file.exists() {
            return Err(Error::new(ErrorKind::NotFound, "File not exist {config_json_file}"))
        }
        
        
        Ok(fs::read_to_string(config_json_file)?)
    }

    pub fn store_config_json(&self, email: &String, config_json: &String) -> Result<(), Error> {

        if !self.dir_path.exists() {
            return Err(Error::new(ErrorKind::NotFound, "Dir not exist {dir_path}"))
        }

        let email = sha512_encrypt(email);

        let mut config_json_file = self.dir_path.clone();
        config_json_file.push(email);
        config_json_file.set_extension("json");
        
        writeln!(File::create(config_json_file)?, "{config_json}")?;
        
        Ok(())
    }

    #[allow(dead_code)]
    pub fn remove_config_json(&self, email: &String) -> Result<(), Error> {
                let email = sha512_encrypt(email);

        let mut config_json_file = self.dir_path.clone();
        config_json_file.push(email);
        config_json_file.set_extension("json");
        
        let path = Path::new(&config_json_file);

        if path.exists() && path.is_file() {
            fs::remove_file(path)?;
        } else {
            return Err(Error::new(ErrorKind::NotFound, "Impossible remove configuration file"));
        }
            
        Ok(())
    }

}