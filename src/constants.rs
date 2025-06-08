

pub mod socket {
    pub const IP : &str = "127.0.0.1";
    pub const PORT :u16 = 10010;
    pub const SSL_CERT :&str = "ssl_cert.pem";
    pub const SSL_KEY : &str = "ssl_key.pem";
}

pub mod data {
    pub const DATA_FOLDER : &str = ".pocket-web-backend";
    pub const DATA_FILE : &str = "data.json";
}


pub(crate) enum Stats {
    Ready = 0,
    Busy,
    UserNotFound = 600,
    WrongSizeToken = 601,
    DeviceIdNotMatch = 602,
    DeviceNotFound = 603,
    SecretNotMatch = 604,
    PasswdError = 605,
    TimestampLastUpdateNotMatch = 606,
    CacheNotFound = 607,
    SecretEmpty = 608,
    TimestampLastNotParsable = 609,
    Error = 700,
    JsonParsingError = 701,
    DbGroupError = 702,
    DbGroupFieldError = 703,
    DbFieldError = 704,
    DbGenericError = 705,
    NoNetwork = 706,
    MapIdError = 707,
    LocalDeviceIdNotMatch = 800,
    Ok = 200
}