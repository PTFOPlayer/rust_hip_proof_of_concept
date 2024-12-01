extern crate proc_macro;
use std::{fs::{self, File}, io::{Read, Write}, path::Path, process::Command
};
extern crate cargo_emit;
use cargo_emit::warning;
use proc_macro::TokenStream;
#[proc_macro]
pub fn my_macro(item: TokenStream) -> TokenStream {
    if !Path::new("./kernels").exists() {
       fs::DirBuilder::new().create("./kernels").unwrap();
    }
    let out_dir = "./kernels".to_owned();
    let file_path = out_dir.clone()+"/file.cpp";
    let lib_path = out_dir.clone()+"/libfile.so";
    let mut file = match File::create(file_path.clone()) {
        Ok(res) => res,
        Err(err) => panic!("file creation error ~: {}", err),
    };

    let data = item.to_string();
    let final_data = &data[3..data.len() - 2];

    _ = file.write(final_data.as_bytes());

    _ = file.flush();

    let mut binding = Command::new("hipcc");
    let cmd = binding
        .args([&file_path, "-shared", "-fPIC", "-o", &lib_path])
        .spawn().expect("compilation error");

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
    let out = String::from("(unsafe {libloading::Library::new(\"") + &lib_path + "\")})"; 
    warning!("{}", out);
    out.parse().expect("parse error")
}
