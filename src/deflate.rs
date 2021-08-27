use flate2::{read::DeflateDecoder, write::DeflateEncoder, Compression};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
};

pub fn compress() -> io::Result<()> {
    let file = File::create("compressed")?;
    let mut e = DeflateEncoder::new(file, Compression::fast());
    let file = fs::read("test")?;
    let _ = e.write_all(&file);
    e.finish()?;
    println!("Compressed");
    Ok(())
}

pub fn decompress(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let mut d = DeflateDecoder::new(file);
    let mut buf = String::new();
    d.read_to_string(&mut buf)?;
    File::create("decompressed")?.write(buf.as_bytes())?;
    Ok(())
}
