use std::env;
use std::path::Path;

fn main() {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let object_path = Path::new(&manifest_dir).join("bin/skirk.o");
  if !object_path.exists() {
    panic!("A core component of building skirk was not found. Aborting.");
  }
  println!("cargo:rustc-link-arg={}", object_path.display());
  println!("cargo:rerun-if-changed=bin/skirk.o");
}
