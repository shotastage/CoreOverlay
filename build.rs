fn main() {
    println!("Generating bindings for C/C++");

    cbindgen::Builder::new()
        .with_crate(".")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/engine.h");
}
