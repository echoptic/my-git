use flate2::{read::DeflateDecoder, write::DeflateEncoder, Compression};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
};

pub fn compress(data: Vec<u8>) -> io::Result<Vec<u8>> {
    let mut contents = Vec::new();
    let mut e = DeflateEncoder::new(&mut contents, Compression::fast());
    e.write_all(&data)?;
    e.finish()?;

    Ok(contents)
}

pub fn decompress(path: &str) -> io::Result<String> {
    let file = File::open(path)?;
    let mut d = DeflateDecoder::new(file);
    let mut buf = String::new();
    d.read_to_string(&mut buf)?;
    let buf = buf.split_once('\0').unwrap().1;

    // Probably better to return bytes by using read_to_end
    Ok(buf.to_string())
}
