extern crate proc_macro;
use std::{
    fs::File,
    io::{Read, Write},
    process::Command,
};
extern crate cargo_emit;
use cargo_emit::warning;
use proc_macro::TokenStream;
#[proc_macro]
pub fn my_macro(item: TokenStream) -> TokenStream {
    let mut file = match File::create("./file.cpp") {
        Ok(res) => res,
        Err(err) => panic!("file creation error ~: {}", err),
    };

    let data = item.to_string();
    let final_data = &data[3..data.len() - 2];

    _ = file.write(final_data.as_bytes());

    _ = file.flush();

    let mut binding = Command::new("hipcc");
    let cmd = binding
        .args(["file.cpp", "-shared", "-fPIC", "-o", "./libfile.so"])
        .spawn()
        .unwrap();

    if let Some(mut stdout) = cmd.stdout {
        let mut s = String::new();
        _ = stdout.read(unsafe { s.as_bytes_mut() });
        warning!("{}", s);
    }
    if let Some(mut stderr) = cmd.stderr {
        let mut s = String::new();
        _ = stderr.read(unsafe { s.as_bytes_mut() });
        warning!("{}", s);
    }

    "(unsafe {libloading::Library::new(\"./libfile.so\")})".parse().unwrap()
}
