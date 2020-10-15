//fn main() { println!("rustc-link-lib=db2"); }
// c_from_rust/build.rs

/*fn main() {
    //println!("rustc-link-lib=db2");
    cc::Build::new()
        .file("mystrlen.c")
        .static_flag(true)
        .compile("mystrlen");

}*/

//use std::env;

fn main() {
    //let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search=C:\\Personal\\R\\clidriver\\bin"); // the "-L" flag
    println!("cargo:rustc-link-lib=db2app64"); // the "-l" flag
}

