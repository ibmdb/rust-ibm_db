#![allow(unused_assignments)]
extern crate bitness;

use std::env;
use std::path::Path;
use bitness::Bitness;
use std::fs::File;
use std::io::{copy, Write, stdout, stdin};
use tempfile::Builder;
use futures::executor::block_on;
use std::fs;
use std::io;
use flate2::read::GzDecoder;
use tar::Archive;

//Main function called for setup
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    let mut env_var_not_present = false;
    env_var_not_present = env::var("IBM_DB_HOME").is_err();
    let mut user_env_path = String::new();
    if env_var_not_present {
        print!("Please enter the path where you need to download the cli driver binary and set as IBM_DB_HOME environment variable (LEAVE BLANK FOR CURRENT DIRECTORY): ");
        let _=stdout().flush();
        stdin().read_line(&mut user_env_path).expect("Did not enter a correct string");
        if let Some('\n')=user_env_path.chars().next_back() {
            user_env_path.pop();
        }
        if let Some('\r')=user_env_path.chars().next_back() {
            user_env_path.pop();
        }
        if user_env_path.len()==0 {
            user_env_path = ".".parse().unwrap();
        }
        println!("Path Entered by you: {}",user_env_path);
    }
    let cli_path: &str = "/clidriver";
    let env_path = env::var("IBM_DB_HOME").unwrap_or(user_env_path);
    let mut value = String::new();
    value.push_str(&env_path);
    value.push_str(cli_path);
    let os = sys_info::os_type().unwrap_or("none".to_string());
    let mut cli_file_name = "";
    println!("clidriver path: {}",value);
    if env_var_not_present {
        if Path::new(&value).exists() {
            println!("clidriver is already present in this path: {}", value);
            //Add to IBM_DB_HOME environment variable
            env::set_var("IBM_DB_HOME", &env_path);
            //Ask user to add to PATH
            println!("Please add this path to PATH & IBM_DB_HOME environment variable if not set.");
            std::process::exit(0)
        }else {
            //If os is windows, check if the os is 32 bit or 64 bit
            if os.contains("Windows") {
                let bitness = bitness::os_bitness().unwrap();
                let os_arch = match bitness {
                    Bitness::X86_32 => 32,
                    Bitness::X86_64 => 64,
                    _ => { 0 }
                };

                if os_arch == 64 {
                    cli_file_name = "ntx64_odbc_cli.zip";
                } else {
                    cli_file_name = "ntx32_odbc_cli.zip";
                }
                println!("This is a {} os & {} bit system. Download file is: {}", os, os_arch, cli_file_name);
            }else if os.contains("Darwin"){
                //If OS is Mac(Darwin), check corresponding arch and find the approp binary
                let bitness = bitness::os_bitness().unwrap();
                let os_arch = match bitness {
                    Bitness::X86_32 => 32,
                    Bitness::X86_64 => 64,
                    _ => { 0 }
                };
                println!("This is a {} os & {} bit system.", os, os_arch);
                if os_arch == 64 {
                    cli_file_name = "macos64_odbc_cli.tar.gz";
                } else {
                    println!("Unknown/Unsupported platform.");
                    std::process::exit(0);
                }

            }else if os.contains("Linux"){
                //If OS is LINUX, check corresponding arch and find the approp binary
                let bitness = bitness::os_bitness().unwrap();
                let os_arch = match bitness {
                    Bitness::X86_32 => 32,
                    Bitness::X86_64 => 64,
                    _ => { 0 }
                };
                println!("This is a {} os & {} bit system.", os, os_arch);
                if os_arch == 64 {
                    cli_file_name = "linuxx64_odbc_cli.tar.gz";
                } else if os_arch == 32{
                    cli_file_name = "linuxx32_odbc_cli.tar.gz";
                }else {
                    println!("Unknown/Unsupported platform.");
                    std::process::exit(0);
                }
            }else{
                println!("Unknown/Unsupported platform.");
                std::process::exit(0);
            }

            //Download the binary
            let mut file_url = "https://public.dhe.ibm.com/ibmdl/export/pub/software/data/db2/drivers/odbc_cli/".to_string();
            file_url.push_str(cli_file_name);

            println!("Downloading at {} from {} .......", env_path, file_url);

            let future =
                download_file(&*env_path, &*file_url);
            let err = block_on(future);

            //Check if Download Successful.
            //If error print error and details
            if !err.is_ok() {
                println!("Error while downloading file: {}", err.err().unwrap());
                std::process::exit(4)
            }

            //Unzip the downloaded binary
            let mut unzip_err = 0;
            if os.contains("Windows") {
                unzip_err = un_zipping(&*env_path, &*cli_file_name);
            }else{
                unzip_err = linux_untar(&*env_path, &*cli_file_name);
            }
            //Check if unzipping Successful.
            //If error print error and details
            if !(unzip_err == 0) {
                println!("Error while unzipping file");
                std::process::exit(4)
            }
            //Set the environment variable
            env::set_var("IBM_DB_HOME", env_path);
            //Validate and then exit
            let env_path_tmp = env::var("IBM_DB_HOME").unwrap_or("Unable to Set Path. Please set.".to_string());
            println!("IBM_DB_HOME set to {}",env_path_tmp);
        }
    } else {
        if os.contains("Windows") {
            if Path::new(&value).exists() {
                println!("clidriver is already present in this path: {}", value);
                //Ask user to add to PATH
                println!("Please add this path to PATH environment variable if not set.");
                std::process::exit(0)
            }else{
                //Follow the above steps of downloading the necessary binary file for windows.
                let bitness = bitness::os_bitness().unwrap();
                let os_arch = match bitness {
                    Bitness::X86_32 => 32,
                    Bitness::X86_64 => 64,
                    _ => { 0 }
                };

                if os_arch == 64 {
                    cli_file_name = "ntx64_odbc_cli.zip";
                } else {
                    cli_file_name = "ntx32_odbc_cli.zip";
                }
                println!("This is a {} os & {} bit system. Download file is: {}", os, os_arch, cli_file_name);

                //Download the binary
                let mut file_url = "https://public.dhe.ibm.com/ibmdl/export/pub/software/data/db2/drivers/odbc_cli/".to_string();
                file_url.push_str(cli_file_name);

                println!("Downloading at {} from {} .......", env_path, file_url);

                let future = download_file(&*env_path, &*file_url);
                let err = block_on(future);

                //Check if Download Successful.
                //If error print error and details
                if !err.is_ok() {
                    println!("Error while downloading file: {}", err.err().unwrap());
                    std::process::exit(4)
                }

                //Unzip the downloaded binary
                let mut unzip_err = 0;
                if os.contains("Windows") {
                    unzip_err = un_zipping(&*env_path, &*cli_file_name);
                }else{
                    unzip_err = linux_untar(&*env_path, &*cli_file_name);
                }

                //Check if unzipping Successful.
                //If error print error and details
                if !(unzip_err == 0) {
                    println!("Error while unzipping file");
                    std::process::exit(4)
                }
                //Validate and then exit
                let env_path_tmp = env::var("IBM_DB_HOME").unwrap_or("Unable to Set Path. Please set.".to_string());
                println!("IBM_DB_HOME set to {}",env_path_tmp);
            }
        } else {
            if Path::new(&value).exists() {
                println!("clidriver is already present in this path: {}", value);
                //Ask user to add to PATH
                println!("Please set CGO_CFLAGS, CGO_LDFLAGS and LD_LIBRARY_PATH or DYLD_LIBRARY_PATH environment variables");
                std::process::exit(0)
            }else{
                //Follow the above steps of downloading the necessary binary file for linux or mac.
                //*************
                if os.contains("Darwin"){
                    //If OS is Mac(Darwin), check corresponding arch and find the approp binary
                    let bitness = bitness::os_bitness().unwrap();
                    let os_arch = match bitness {
                        Bitness::X86_32 => 32,
                        Bitness::X86_64 => 64,
                        _ => { 0 }
                    };
                    println!("This is a {} os & {} bit system.", os, os_arch);
                    if os_arch == 64 {
                        cli_file_name = "macos64_odbc_cli.tar.gz";
                    } else {
                        println!("Unknown/Unsupported platform.");
                        std::process::exit(0);
                    }

                }else if os.contains("Linux"){
                    //If OS is LINUX, check corresponding arch and find the approp binary
                    let bitness = bitness::os_bitness().unwrap();
                    let os_arch = match bitness {
                        Bitness::X86_32 => 32,
                        Bitness::X86_64 => 64,
                        _ => { 0 }
                    };
                    println!("This is a {} os & {} bit system.", os, os_arch);
                    if os_arch == 64 {
                        cli_file_name = "linuxx64_odbc_cli.tar.gz";
                    } else if os_arch == 32{
                        cli_file_name = "linuxx32_odbc_cli.tar.gz";
                    }else {
                        println!("Unknown/Unsupported platform.");
                        std::process::exit(0);
                    }
                }else{
                    println!("Unknown/Unsupported platform.");
                    std::process::exit(0);
                }
                //Download the binary
                let mut file_url = "https://public.dhe.ibm.com/ibmdl/export/pub/software/data/db2/drivers/odbc_cli/".to_string();
                file_url.push_str(cli_file_name);

                println!("Downloading at {} from {} .......", env_path, file_url);

                let future =
                    download_file(&*env_path, &*file_url);
                let err = block_on(future);

                //Check if Download Successful.
                //If error print error and details
                if !err.is_ok() {
                    println!("Error while downloading file: {}", err.err().unwrap());
                    std::process::exit(4)
                }

                //Unzip the downloaded binary
                let mut unzip_err = 0;
                unzip_err = linux_untar(&*env_path, &*cli_file_name);

                //Check if unzipping Successful.
                //If error print error and details
                if !(unzip_err == 0) {
                    println!("Error while unzipping file");
                    std::process::exit(4)
                }
                    //***************
            }
        }

    }

}

//Function to download the platform specific file
async fn download_file(env_path: &str, file_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\n****************************************\n\
            You are downloading a package which to be used by RUST module for IBM DB2/Informix.  \
            The module is licensed under the Apache License 2.0. \
            The package also includes IBM ODBC and CLI Driver from IBM, \
            which is automatically downloaded as the platform Binary and is copied on your system/device. \
            The license agreement to the IBM ODBC and CLI Driver is available in {}\
            /clidriver/license. Check for additional dependencies, \
            which may come with their own license agreement(s). \
            Your use of the components of the package and dependencies constitutes your \
            cceptance of their respective license agreements. \
            If you do not accept the terms of any license agreement(s), \
            then delete the relevant component(s) from your device.\n****************************************\n"
             ,env_path);
    let mut response = reqwest::get(file_url).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");


        //Check if the zip file already exists
        if !Path::new(&env_path).join(fname).exists(){
            println!("file to download: '{}'", fname);
            File::create(Path::new(&env_path).join(fname))?
        } else {
            println!("file {} already exists.", fname);
            return Ok(())
        }
    };
    let content =  response.bytes().await?;
    dest.write_all(&*content)?;
    Ok(())
}

//Unzipping the file on windows
fn un_zipping(env_path: &str, cli_file_name: &str) -> i32{
    if Path::new(env_path).join("/clidriver").exists() {
        println!("clidriver already downloaded and unzipped.");
        return 0;
    }
    let mut fname = String::from(env_path);
    fname.push_str(cli_file_name);
    println!("Unzipping {}....",fname);

    let fname = std::path::Path::new(env_path).join(cli_file_name);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();
    let copy_path = std::path::Path::new(env_path);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpathtmp = file.name();
        let outpath = copy_path.join(outpathtmp);

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!("extracted to \"{}\"", outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "exxtracted to \"{}\" ({} bytes)",
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    return 0;
}

//Unzipping the file on Linux/UNIX
fn linux_untar(env_path: &str, cli_file_name: &str)-> i32{
    if Path::new(env_path).join("/clidriver").exists() {
        println!("clidriver already downloaded and unzipped.");
        return 0;
    }
    let fname = std::path::Path::new(env_path).join(cli_file_name);
    println!("Untarring {}....", fname.display());
    let file = fs::File::open(&fname).unwrap();
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    archive.unpack(env_path);

    return 0;
}