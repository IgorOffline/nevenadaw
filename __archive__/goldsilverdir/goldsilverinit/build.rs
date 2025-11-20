use std::path::PathBuf;

fn main() {
    let lib_path = PathBuf::from("./libs")
        .canonicalize()
        .expect("Could not canonicalize library path");

    println!(
        "cargo:rustc-link-search=native={}",
        lib_path.to_str().unwrap()
    );

    println!("cargo:rustc-link-lib=dylib=SDL2");
    println!("cargo:rustc-link-lib=dylib=SDL2_ttf");
}
