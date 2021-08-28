use std::env;

use crate::{blob::Blob, index::Index};

pub fn checkout(path: &Option<&str>) {
    if let Some(path) = path {
        println!("restoring in {}", path)
    } else {
        let cwd = env::current_dir().unwrap();
        let mut index = Index::new(&cwd).unwrap();
        for (path, hash) in index.hashtree.iter() {
            let mut prefix = String::new();
            let file_path = cwd.join(path);
            if !file_path.exists() {
                prefix += "D";
            } else {
                let blob = Blob::from_path(file_path).unwrap();
                if hash != &blob.hash {
                    prefix += "M";
                }
            }
            if !prefix.is_empty() {
                println!("{}\t\t{}", prefix, path);
            }
        }
    }
}
