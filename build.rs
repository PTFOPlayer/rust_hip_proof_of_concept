use std::{io::Read, process::Command};

fn main() {
    let mut binding = Command::new("hipcc");
    let cmd = binding
        .args(["./src/wrapper/wrapper.cpp", "-shared", "-fPIC", "-o", "./src/wrapper/libwrapper.so"])
        .spawn()
        .unwrap();

    if let Some(mut stdout) = cmd.stdout {
        let mut s = String::new();
        _ = stdout.read(unsafe { s.as_bytes_mut() });
        println!("cargo::warning={}", s);
    }
    if let Some(mut stderr) = cmd.stderr {
        let mut s = String::new();
        _ = stderr.read(unsafe { s.as_bytes_mut() });
        panic!("{}", s);
    }

    std::fs::copy("src/wrapper/libwrapper.so", "./target/release/libwrapper.so").unwrap();

    println!("cargo::rustc-link-search=native=src/wrapper");
    println!("cargo::rustc-link-lib=wrapper");
}