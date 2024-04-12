use hip_macros::*;

use libloading;

fn main() {
    let a = my_macro!(
      extern "C" void hello_from_macro()
      {
        printf("hello from macro \n");
      }
    );

    unsafe {
        a.0.get::<unsafe extern "C" fn()>(b"hello_from_macro")
            .unwrap()()
    };
}
