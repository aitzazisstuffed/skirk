use std::ffi::CStr;
use std::os::raw::c_char;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--version" {
        unsafe {
            let version_ptr = skirk_version();
            if !version_ptr.is_null() {
                let c_str = CStr::from_ptr(version_ptr);
                if let Ok(str_slice) = c_str.to_str() {
                    println!("{}", str_slice);
                } else {
                    eprintln!("Failed to convert version string");
                }
            } else {
                eprintln!("Version pointer was null");
            }
        }
    } else {
        println!("Usage: skirk --version");
    }
}

extern "C" {
    fn skirk_version() -> *const c_char;
}
