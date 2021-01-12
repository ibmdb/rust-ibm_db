#![allow(unused_assignments)]
use std::env;

fn main() {
    let env_var_not_present = env::var("IBM_DB_HOME").is_err();
	let mut project_dir = String::new();
	if env_var_not_present {
		project_dir = ".".parse().unwrap();
	}else{
		project_dir = env::var("IBM_DB_HOME").unwrap();
	}
    println!("cargo:rustc-link-search={}/{}",project_dir,"lib"); // the "-L" flag
    println!("cargo:rustc-link-lib=db2"); // the "-l" flag
    //Change this to db2app64 for Windows.
    //println!("cargo:rustc-link-lib=db2app64"); // the "-l" flag
}

