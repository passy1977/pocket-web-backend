
pub(crate) mod conf {
    pub const IP: &str = "127.0.0.1";
    pub const PORT :u16 = 8080;
}

pub(crate) mod jwt {
    pub const JWT_ISS: &str = "http://localhost:8080";
    pub const JWT_AUD: &str = "http://localhost:8080";
    
    pub const SECRET:  &str = "verysecretsecret";
}

pub(crate)  mod data {
    pub const DATA_FOLDER : &str = ".pocket-web-backend";
    pub const DATA_FILE : &str = "data.json";

    pub const EMPTY_CONFIG_JSON : &str = "{}";
}


// pub(crate) enum Stats {
//     Ready = 0,
//     Busy,
//     UserNotFound = 600,
//     WrongSizeToken = 601,
//     DeviceIdNotMatch = 602,
//     DeviceNotFound = 603,
//     SecretNotMatch = 604,
//     PasswdError = 605,
//     TimestampLastUpdateNotMatch = 606,
//     CacheNotFound = 607,
//     SecretEmpty = 608,
//     TimestampLastNotParsable = 609,
//     Error = 700,
//     JsonParsingError = 701,
//     DbGroupError = 702,
//     DbGroupFieldError = 703,
//     DbFieldError = 704,
//     DbGenericError = 705,
//     NoNetwork = 706,
//     MapIdError = 707,
//     LocalDeviceIdNotMatch = 800,
//     Ok = 200
// }