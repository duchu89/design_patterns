mod composit;

use composit::{AbstractFile, File, Directory};

pub fn test() {
    let mut projects = Directory::new("Projects".to_string());
    let mut c_plus_plus = Directory::new("C++".to_string());
    let mut rust = Directory::new("Rust".to_string());
    let mut hello_world_in_c = Directory::new("Hello World".to_string());
    let mut hello_world_in_rust = Directory::new("Hello_world".to_string());
    let hello_world_rust = File{name:"main.rs".to_string()};
    let hello_world_c = File{name:"main.cc".to_string()};
    let cargo_rust = File{name:"cargo.toml".to_string()};
    let makefile = File{name:"makefile.txt".to_string()};
    let index = File{name:"index.txt".to_string()};

    hello_world_in_c.add(Box::new(hello_world_c));
    hello_world_in_c.add(Box::new(makefile));

    hello_world_in_rust.add(Box::new(hello_world_rust));
    hello_world_in_rust.add(Box::new(cargo_rust));

    c_plus_plus.add(Box::new(hello_world_in_c));

    rust.add(Box::new(hello_world_in_rust));

    projects.add(Box::new(c_plus_plus));
    projects.add(Box::new(rust));
    projects.add(Box::new(index));

    projects.ls();
}