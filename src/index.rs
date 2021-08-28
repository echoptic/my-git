use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};

pub struct Index {
    path: PathBuf,
    pub hashtree: BTreeMap<String, String>,
}

impl Index {
    pub fn new(path: &PathBuf) -> io::Result<Self> {
        let path = path.join(".got").join("index");
        let mut index = Index {
            path,
            hashtree: BTreeMap::new(),
        };

        if !index.path.exists() {
            return Ok(index);
        }

        let index_file = File::open(&index.path)?;
        let contents = BufReader::new(&index_file);
        for line in contents.lines() {
            let line = line?;
            let (path, hash) = line.split_once(' ').unwrap();
            index.update(path.into(), hash)?;
        }

        Ok(index)
    }

    pub fn update(&mut self, path: &str, hash: &str) -> io::Result<()> {
        self.hashtree.insert(path.to_string(), hash.to_string());

        Ok(())
    }

    pub fn write(&self) -> io::Result<()> {
        let mut index = File::create(&self.path)?;
        println!("index path: {}", self.path.display());
        for (path, hash) in self.hashtree.iter() {
            writeln!(&mut index, "{} {}", path, hash)?;
        }

        Ok(())
    }
}
