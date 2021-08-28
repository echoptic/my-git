use std::{
    env,
    fs::{self, File},
    io::{self, Write},
};

use crate::{blob::Blob, deflate::compress, index::Index};

// TODO: Handle when given directory
pub fn add(files: &Vec<&str>) -> io::Result<()> {
    let cwd = env::current_dir()?;
    let objects_dir = cwd.join(".got").join("objects");

    let mut index = Index::new(&cwd)?;

    let add_blob = |blob: Blob| -> io::Result<()> {
        // This creates a folder in objects with the first 2 digits of hash
        // as the folder name and rest of digits as a file name inside of it
        let object_folder = objects_dir.join(&blob.hash[..2]);
        let object_file = object_folder.join(&blob.hash[2..]);
        // Ignore if the directory already exists
        let _ = fs::create_dir(&object_folder);
        let mut file = File::create(&object_file)?;
        let blob_data = compress(&blob.data)?;
        println!("writing from blob data: {:?}", &blob_data);
        file.write_all(&blob_data)?;

        Ok(())
    };

    for file in files {
        let file_path = cwd.join(file);
        let blob = Blob::from_path(file_path.clone())?;
        index.update(
            file_path.strip_prefix(&cwd).unwrap().to_str().unwrap(),
            &blob.hash,
        )?;
        add_blob(blob)?;
    }
    index.write()?;

    Ok(())
}
