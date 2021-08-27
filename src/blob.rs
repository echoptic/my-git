use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use sha1::Sha1;

pub struct Blob {
    pub hash: String,
    pub data: Vec<u8>,
}

impl Blob {
    pub fn from_path(path: &str) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        let hash = Sha1::from(&data).digest().to_string();

        Ok(Self { hash, data })
    }
}
