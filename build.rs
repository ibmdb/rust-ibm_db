
fn main() {
    println!("cargo:rustc-link-search=<path to CLI Driver Bin>\\clidriver\\bin"); // the "-L" flag
    println!("cargo:rustc-link-lib=db2app64"); // the "-l" flag
}

