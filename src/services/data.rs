use crate::constants::{data::*, conf::*};
use crate::utils::{sha512_encrypt, Result};
use crate::services::cli::Cli;
use std::path::{Path, PathBuf};
use std::{env, fs};
use std::fs::File;
use std::io::{self, Error, ErrorKind, Read, Write};
use serde::{Deserialize, Serialize};

pub struct Url {
    pub scheme: String,
    pub address: String,
    pub port: Option<u16>,
}

impl Default for Url {
    fn default() -> Self {
        Self {
            scheme: "http".to_string(),
            address: "localhost".to_string(),
            port: None,
        }
    }
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Data {

    #[serde(skip_serializing, skip_deserializing)]
    file_data_path: PathBuf,

    #[serde(skip_serializing, skip_deserializing)]
    pub dir_path: PathBuf,

    /// Address on which the server listens
    pub address: String,

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
        
        let mut ret = Self {
            file_data_path,
            dir_path,
            address: ADDRESS.to_string(),
            update: false,
            max_threads: MAX_BLOCKING_THREADS,
            session_expiration_time: SESSION_EXPIRATION_TIME
        };
        
        // Try to load existing configuration
        if ret.file_data_path.exists() {
            if let Err(e) = ret.load() {
                ret.update = true;
                eprintln!("Error loading data: {e}");
            }
        } else {
            // File doesn't exist, mark for creation with default values
            ret.update = true;
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
        // Check if file exists
        if !self.file_data_path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Configuration file does not exist"));
        }

        let mut file = File::open(self.file_data_path.as_path())?;

        let mut data = String::new();

        file.read_to_string(&mut data)?;

        // Check if file is empty
        if data.trim().is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Configuration file is empty"));
        }

        let data : Data = serde_json::from_str(&data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Invalid JSON format: {}", e)))?;

        self.address = data.address;
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

    pub fn get_url(&self) -> Result<Url> {
        let mut ret = Url::default();

        if self.address.starts_with("http://") {
            ret.scheme = "http".to_string();
        } else if self.address.starts_with("https://") {
            ret.scheme = "https".to_string();
        } else {
            return Err("Invalid scheme".into());
        }

        let slices = self.address.split(':').collect::<Vec<_>>();
        if slices.len() == 0 || slices.len() > 3 {
            return Err("Invalid address format".into());
        }

        ret.address = slices[1].trim_start_matches("//").to_string();

        if slices.len() == 3 {
            if let Ok(port) = slices[2].parse::<u16>() {
                ret.port = Some(port);
            } else {
                return Err("Invalid port".into());
            }
        }

        Ok(ret)
    }
}