use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

use crate::{blob::Blob, deflate::compress};

pub fn add(files: &Vec<&str>) -> io::Result<()> {
    let cwd = env::current_dir()?;
    let objects_dir = cwd.join(".got").join("objects");

    let add_blob = |blob: Blob| -> io::Result<()> {
        // This creates a folder in objects with the first 2 digits of hash
        // as the folder name and rest of digits as a file name inside of it
        let folder_path = objects_dir.join(&blob.hash[..2]);
        let file_path = folder_path.join(&blob.hash[2..]);
        match fs::create_dir(&folder_path) {
            Ok(_) => {}
            Err(_) => {}
        };
        let mut file = File::create(&file_path)?;
        let blob_data = compress(blob.data)?;
        println!("writing from blob data: {:?}", blob_data);
        file.write_all(&blob_data)?;

        Ok(())
    };

    for file in files {
        let file_path = cwd.join(file);
        let blob = Blob::from_path(file_path.clone())?;
        add_blob(blob)?;
    }

    Ok(())
}
