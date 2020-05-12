pub trait AbstractFile {
    fn ls(&self);
}

pub struct File {
    pub name: String,
}

impl AbstractFile for File {
    fn ls(&self) {
        println!("File name: {}", self.name);
    }
}

pub struct Directory {
    name: String,
    files: Vec<Box<dyn AbstractFile>>,
}

impl Directory {
    pub fn new(name: String) -> Directory {
        Directory {
            name: name,
            files: Vec::new(),
        }
    }

    pub fn add(&mut self, file: Box<dyn AbstractFile>) {
        self.files.push(file);
    }
}

impl AbstractFile for Directory {
    fn ls(&self) {
        println!("Directory name: {}, contains: ", self.name);
        for file in self.files.iter() {
            file.ls();
        }
    }
}
