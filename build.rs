
fn main() {
    let project_dir = env::var("IBM_DB_HOME").unwrap();
    println!("cargo:rustc-link-search={}/{}",project_dir,"lib"); // the "-L" flag
    println!("cargo:rustc-link-lib=db2"); // the "-l" flag

}

