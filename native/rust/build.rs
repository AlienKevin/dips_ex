fn main() {
    println!("cargo:rustc-link-lib=dylib=bert");
    println!("cargo:rustc-link-search=native=lib");
    // https://stackoverflow.com/a/46699544
    println!("cargo:rustc-link-arg=-Wl,-rpath,native/rust/lib");
}