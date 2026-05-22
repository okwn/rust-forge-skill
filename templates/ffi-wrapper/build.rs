fn main() {
    // Tell Cargo to re-run if header changes
    println!("cargo:rerun-if-changed=src-c/lib.h");

    // Compile the C library
    cc::Build::new()
        .file("src-c/lib.c")
        .compile("my_clib");

    println!("cargo:rustc-link-lib=static=my_clib");
    println!("cargo:rustc-link-search=native=target/release/deps");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("src-c/lib.h")
        .use_core()
        .prepend_type_name(true)
        .ctypes_prefix("::ctypes")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}