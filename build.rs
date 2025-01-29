use std::{path::Path, process::Command};

fn main() {
    compile_kernel(Path::new("./kernels/file.cpp"));
}

fn compile_kernel(file_path: &Path) {
    let file_stem = file_path.file_stem().unwrap();
    let dir = file_path.parent().unwrap();
    let file = file_path.to_str().unwrap();
    
    let out = dir.to_str().unwrap().to_string() + "/lib" + file_stem.to_str().unwrap() + ".so";

    let mut binding = Command::new("hipcc");
    let cmd = binding
        .args([file, "-shared", "-fPIC", "-o", &out])
        .output()
        .unwrap();
    if cmd.stderr.len() != 0 {
        panic!("{}", String::from_utf8_lossy(&cmd.stderr));
    }
    println!("cargo::warning={:?}", cmd);
}
