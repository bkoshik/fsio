use std::io::{Error as IoError, ErrorKind};
use serde::de::DeserializeOwned;
use crate::{
    errors::*,
    file::File
};

impl File {
    pub fn json<T>(&mut self) -> Result<T>
    where
        T: DeserializeOwned
    {
        return serde_json::from_str(&self.read_from_start()?)
            .map_err(|e| Error::Io(IoError::new(ErrorKind::Other, e)));
    }

    pub fn yaml<T>(&mut self) -> Result<T>
    where
        T: DeserializeOwned
    {
        return serde_yaml::from_str(&self.read_from_start()?)
            .map_err(|e| Error::Io(IoError::new(ErrorKind::Other, e)));
    }

    pub fn toml<T>(&mut self) -> Result<T>
    where
        T: DeserializeOwned
    {
        return toml::from_str(&self.read_from_start()?)
            .map_err(|e| Error::Io(IoError::new(ErrorKind::Other, e)));
    }
}