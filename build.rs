
fn main() {
    println!("cargo:rustc-link-search=/home/clidriver/lib"); // the "-L" flag
    println!("cargo:rustc-link-lib=db2"); // the "-l" flag
}

