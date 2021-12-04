extern crate libc;

use std::ffi::{CStr, CString};
use std::time::Instant;

#[link(name = "json", kind = "static")]
extern "C" {
    // fn doubleX(x: libc::c_int) -> libc::c_int;
    // fn sayHello(s: *const libc::c_char);
    fn formatJSON(filename: *const libc::c_char, indent: libc::c_int);
}

fn main() {

    unsafe {
        /*let x = String::from("chen");
        let x = CString::new("chenxx").unwrap();
        sayHello(CString::new("chen").unwrap().as_ptr());
        sayHello(CString::new("jie").unwrap().as_ptr());*/
        println!("start...");
        let start = Instant::now();
        formatJSON(
            CString::new("abc.json").unwrap().as_ptr(),
            2
        );
        println!("cost: {:?}", start.elapsed());
    };
}
