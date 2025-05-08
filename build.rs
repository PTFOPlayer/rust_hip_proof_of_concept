use std::{fs, path::Path, process::Command};

fn main() {
    let kernel_dir = Path::new("kernel_sources");

    if !kernel_dir.exists() {
        return;
    }

    for entry in fs::read_dir(kernel_dir).unwrap() {
        let Ok(entry) = entry else {
            continue;
        };

        let entry = entry.path();
        let entry = entry.to_str().unwrap();
        let entry = "./".to_owned() + entry;

        let status = Command::new("sh")
            .arg("-c")
            .arg("cd ./kernel_sources/kernel/; cargo build --release; cd ./../../;")
            .status()
            .expect("Failed to execute cargo build");
        if !status.success() {
            panic!("Kernel compilation failed for {:?}", entry);
        }
    }
}
