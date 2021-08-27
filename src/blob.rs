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
    pub fn from_path(path: PathBuf) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        // This is the blob (file len)\0 at start of every git file
        let mut blob_info = Vec::new();
        format!("blob {}\0", data.len())
            .as_bytes()
            .iter()
            .for_each(|&byte| blob_info.push(byte));

        blob_info.append(&mut data);

        let hash = Sha1::from(&blob_info).digest().to_string();

        Ok(Self {
            hash,
            data: blob_info,
        })
    }
}
