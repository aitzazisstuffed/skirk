fn main(){
    println!("Version 0.0.1");
}

unsafe extern "C" {
    fn skirk_version() -> *const std::os::raw::c_char;
}
