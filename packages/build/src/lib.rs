pub fn sysroot() -> String {
    let cwd = std::env::current_dir().unwrap();
    let root = std::fs::canonicalize(cwd.join("../..")).unwrap();
    root.join("build/sysroot").display().to_string()
}

pub fn add_sysroot_lib_path() {
    let sysroot = sysroot();

    println!(
        "cargo:rustc-link-arg=--sysroot={sysroot}",
    );
}
