#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
     println!("cargo:rustc-flags=-L../opcodes -l opcodes");
    // println!("cargo:rustc-flags=-l readline");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
