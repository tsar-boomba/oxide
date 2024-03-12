pub fn sysroot() -> String {
    let cwd = std::env::current_dir().unwrap();
    let root = std::fs::canonicalize(cwd.join("../..")).unwrap();
    root.join("build/sysroot").display().to_string()
}
