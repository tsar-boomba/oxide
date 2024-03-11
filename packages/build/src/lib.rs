pub fn add_sysroot_lib_path() {
    let cwd = std::env::current_dir().unwrap();
    let root = std::fs::canonicalize(cwd.join("../..")).unwrap();
    let sysroot = root.join("build/sysroot").display().to_string();

    println!(
        "cargo:rustc-link-arg=--sysroot={sysroot}",
    );
}
