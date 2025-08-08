use std::io::{
    Error as IoError, 
    ErrorKind
};
use nix::{
    sys::stat::fstat,
    unistd::read
};
use crate::{
    errors::*,
    fileio::FileIO
};

impl FileIO {
    pub fn read(&self) -> Result<String> {
        let len: usize = fstat(&self.file)
            .map_err(|e| Error::Io(IoError::new(ErrorKind::InvalidInput, e)))?
            .st_size as usize;
        let mut content_bytes = vec![0u8; len];

        let n = read(&self.file, &mut content_bytes)
            .map_err(|e| Error::Io(IoError::new(ErrorKind::InvalidData, e)))?;
        content_bytes.truncate(n);

        let content: String = String::from_utf8(content_bytes.to_vec())
            .map_err(|e| Error::Io(IoError::new(ErrorKind::InvalidInput, e)))?;

        return Ok(content);
    }

    pub fn read_lines(&mut self) -> Result<Vec<String>> {
        let mut lines: Vec<String> = Vec::new();
        let content: String = self.read()?;

        for line in content.split('\n') {
            lines.push(line.to_string());
        }

        return Ok(lines);
    }
}