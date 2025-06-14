use crate::constants::{data::*, conf::*, jwt::{JWT_AUD, JWT_ISS}};
use crate::utils::Result;
use crate::services::cli::Cli;
use std::path::{Path, PathBuf};
use std::{env, fs};
use std::fs::File;
use std::io::{self, Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {

    #[serde(skip_serializing, skip_deserializing)]
    file_data_path: PathBuf,

    /// IP address on which the server listens
    pub ip: String,

    /// Connection port
    pub port: u16,

    pub jwt_iss: String,

    pub jwt_aud: String,
    
    #[serde(skip_serializing, skip_deserializing)]
    pub(super) update : bool
    
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
            ip: IP.to_string(),
            port: PORT,
            jwt_iss: JWT_ISS.to_string(),
            jwt_aud: JWT_AUD.to_string(),
            update: false
        };
        
        if let Err(e) = ret.load() {
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
        self.jwt_aud = data.jwt_aud;
        self.jwt_iss = data.jwt_iss;
        self.update = false;

        Ok(())
    }
    pub fn store(&self) -> Result<(), io::Error> {

        let data_json = serde_json::to_string(&self)?;

        let mut file = File::create(self.file_data_path.clone())?;

        file.write_all(data_json.as_bytes())?;

        Ok(())
    }
}