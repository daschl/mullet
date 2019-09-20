use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::env;

extern "C" {
    pub fn RunQuery(q: *const c_char) -> *const c_char;
}

fn main() {
    let query_string = env::args().nth(1).expect("No query provided as arg!");

    println!("Query: {:?}", query_string);

    let query = CString::new(query_string).unwrap();
    unsafe {
        let result = CStr::from_ptr(RunQuery(query.as_ptr()));
        println!("Result: {:?}", result.to_str().unwrap());
    }

}