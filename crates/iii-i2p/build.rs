fn main() {
    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-link-lib=static=i2pd");
    println!("cargo:rustc-link-lib=static=i2pdclient");
    cxx_build::bridge("src/lib.rs")
        .file("src/ffi.cc")
        .compile("iii-i2p-ffi");
}
