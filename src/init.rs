use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

pub fn init(subdir: &Option<&str>) -> io::Result<PathBuf> {
    let cwd = env::current_dir()?;
    let path = match subdir {
        Some(dir) => {
            let subdir = &cwd.join(dir);
            fs::create_dir(&subdir)?;
            cwd.join(&subdir).join(".got")
        }
        None => cwd.join(".got"),
    };

    let path = Path::new(&path);

    fs::create_dir(path)?;
    fs::create_dir(path.join("branches"))?;
    fs::create_dir(path.join("info"))?;
    fs::create_dir(path.join("objects"))?;
    fs::create_dir(path.join("refs"))?;
    fs::create_dir(path.join("refs").join("heads"))?;
    fs::create_dir(path.join("refs").join("tags"))?;

    File::create(path.join("description"))?;
    let mut head = File::create(path.join("HEAD"))?;
    head.write_all("ref: refs/heads/master".as_bytes())?;

    Ok(path.clone().to_path_buf())
}
