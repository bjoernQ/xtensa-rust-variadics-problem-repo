use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    copy_file(&out, "./libs/libloggingesp32.a", "liblogging.a");

    println!("cargo:rustc-link-lib={}", "logging");
    println!("cargo:rustc-link-search={}", out.display());
}

fn copy_file(out: &PathBuf, from: &str, to: &str) {
    let mut file = File::create(out.join(to)).unwrap();
    file.write_all(&fs::read(from).unwrap()).unwrap();
}
