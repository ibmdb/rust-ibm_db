
fn main() {
    println!("cargo:rustc-link-search=C:\\Personal\\R\\clidriver\\bin"); // the "-L" flag
    println!("cargo:rustc-link-lib=db2app64"); // the "-l" flag
}

