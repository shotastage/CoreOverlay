extern crate cbindgen;

fn main() {
    cbindgen::Builder::new()
      .with_crate(".")
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("include/engine.h");
}
