extern crate proc_macro;
use std::{fs::File, io::Write, process::Command, str::FromStr};
extern crate cargo_emit;
use libloading;
use proc_macro::TokenStream;
#[proc_macro]
pub fn my_macro(item: TokenStream) -> TokenStream {
    let mut file = File::options()
        .write(true)
        .create(true)
        .open("file.cpp")
        .unwrap();

    _ = file.write(item.to_string().as_bytes());

    let mut cmd = Command::new("/opt/rocm/bin/hipcc");
    cmd.args(["file.cpp", "-shared", "-fPIC", "-o", "libfile.so"]);
    let out = match cmd.output() {
        Ok(ok) => ok,
        Err(err) => panic!("{}", err),
    };
    let c_out = format!(
        "{}\n{}",
        String::from_utf8_lossy(out.stdout.as_ref()),
        String::from_utf8_lossy(out.stdout.as_ref())
    );

    let tokens =  "
    (unsafe { 
        libloading::Library::new(\"./libfile.so\").unwrap()
    },\"".to_owned() + &c_out + "\")";

    TokenStream::from_str(&tokens).unwrap()
}
