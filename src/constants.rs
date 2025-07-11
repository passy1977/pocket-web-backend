use std::fmt;
use std::fmt::{Display, Formatter};
use crate::bindings;
use crate::bindings::pocket_stat_t;
use crate::constants::Stats::{Busy, CacheNotFound, DbFieldError, DbGenericError, DbGroupError, DbGroupFieldError, DeviceIdNotMatch, DeviceNotFound, Error, JsonParsingError, LocalDeviceIdNotMatch, MapIdError, NoNetwork, PasswdError, Ready, SecretEmpty, SecretNotMatch, TimestampLastNotParsable, TimestampLastUpdateNotMatch, UserNotFound, WrongSizeToken};

pub(crate) mod conf {
    pub const IP: &str = "127.0.0.1";
    pub const PORT :u16 = 8080;
}

pub(crate)  mod data {
    pub const DATA_FOLDER : &str = ".pocket-web-backend";
    pub const DATA_FILE : &str = "data.json";
}

#[derive(Clone, Copy)]
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

impl Stats {
    pub(crate) fn from(status : pocket_stat_t) -> Self {
        match status {
            bindings::pocket_stat_t_READY => Ready,
            bindings::pocket_stat_t_BUSY => Busy,
            bindings::pocket_stat_t_USER_NOT_FOUND => UserNotFound,
            bindings::pocket_stat_t_WRONG_SIZE_TOKEN => WrongSizeToken,
            bindings::pocket_stat_t_DEVICE_ID_NOT_MATCH => DeviceIdNotMatch,
            bindings::pocket_stat_t_DEVICE_NOT_FOUND => DeviceNotFound,
            bindings::pocket_stat_t_SECRET_NOT_MATCH => SecretNotMatch,
            bindings::pocket_stat_t_PASSWD_ERROR => PasswdError,
            bindings::pocket_stat_t_TIMESTAMP_LAST_UPDATE_NOT_MATCH => TimestampLastUpdateNotMatch,
            bindings::pocket_stat_t_CACHE_NOT_FOND => CacheNotFound,
            bindings::pocket_stat_t_SECRET_EMPTY => SecretEmpty,
            bindings::pocket_stat_t_TIMESTAMP_LAST_NOT_PARSABLE => TimestampLastNotParsable,
            bindings::pocket_stat_t_ERROR => Error,
            bindings::pocket_stat_t_JSON_PARSING_ERROR => JsonParsingError,
            bindings::pocket_stat_t_DB_GROUP_ERROR => DbGroupError,
            bindings::pocket_stat_t_DB_GROUP_FIELD_ERROR=> DbGroupFieldError,
            bindings::pocket_stat_t_DB_FIELD_ERROR => DbFieldError,
            bindings::pocket_stat_t_DB_GENERIC_ERROR => DbGenericError,
            bindings::pocket_stat_t_NO_NETWORK => NoNetwork,
            bindings::pocket_stat_t_MAP_ID_ERROR => MapIdError,
            bindings::pocket_stat_t_LOCAL_DEVICE_ID_NOT_MATCH => LocalDeviceIdNotMatch,
            bindings::pocket_stat_t_OK => Stats::Ok,
            _ => Error
        }
    }
}

impl PartialEq<pocket_stat_t> for Stats {
    fn eq(&self, other: &pocket_stat_t) -> bool {
        (*self as u32) == *other
    }
}

impl PartialEq<Stats> for pocket_stat_t {
    fn eq(&self, other: &Stats) -> bool {
        *self == (*other as u32)
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{})", *self as u32)
    }
}

impl Into<u32> for Stats {
    fn into(self) -> u32 {
        self as u32
    }
}