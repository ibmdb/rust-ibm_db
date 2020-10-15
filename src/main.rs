// c_from_rust/src/main.rs

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, improper_ctypes)]

use ::CCODE::SQLConnect;
use CCODE::{SQLAllocHandle, SQL_HANDLE_DBC, SQL_HANDLE_ENV, SQL_SUCCESS, SQLDisconnect, SQLFreeHandle, SQLGetInfo, SQLSMALLINT, SQL_DATA_SOURCE_NAME, SQLGetFunctions, SQL_API_SQLGETINFO, SQLUSMALLINT, SQL_TRUE};
use std::ffi::CStr;


/*#[link(name = "mystrlen")]
extern "C" {
    fn mystrlen(str: *const c_char) -> c_uint;
}
*/
/*fn safe_mystrlen(str: &str) -> Option<u32> {
    let c_string = match CString::new(str) { 
        Ok(c) => c, 
        Err(_) => return None 
    };

    unsafe { 
        Some(mystrlen(c_string.as_ptr())) 
    } 
}*/

fn main() {

    /*let c_string = CString::new("C From Rust").expect("failed");
    let count = unsafe { 
        mystrlen(c_string.as_ptr()) 
    };*/
    //let c2_string:CCHAR = CCHAR::new("DSN=dashdb;DATABASE=RS22DDS2;hostname=rs22.rocketsoftware.com;PORT=3720;UID=ts5612;PWD=mar@2019;").expect("failed");

    unsafe{
        let mut hdbc= core::ptr::null_mut();
        //let henv:*mut SQLHANDLE = ptr::null_mut();

        let mut dsn = String::new();
        dsn.push_str("dashdb");
        let mut uid = String::new();
        uid.push_str("db2admin");
        let mut pwd = String::new();
        pwd.push_str("gr8tcode!");
        let mut cliRC;
        let mut out = core::ptr::null_mut();
        cliRC = SQLAllocHandle(SQL_HANDLE_ENV as i16,
                               core::ptr::null_mut(),
                               &mut out) ;
        if cliRC!= SQL_SUCCESS as i16 {
            println!("--ERROR while allocating the environment handle. Status: {}",cliRC);
            return;
        }

        cliRC = SQLAllocHandle(SQL_HANDLE_DBC as i16,
                               out,
                               &mut hdbc);
        if cliRC!= SQL_SUCCESS as i16 {
            println!("--ERROR while getting hdbc. Status: {}",cliRC);
            return;
        }
        println!("Connecting to database: {} .........",dsn);
        SQLConnect(
            hdbc
            ,
            dsn.as_mut_ptr()
            ,
            dsn.len() as i16
            ,
            uid.as_mut_ptr()
            ,
            uid.len() as i16
            ,
            pwd.as_mut_ptr()
            ,
            pwd.len() as i16
        );
        println!("Connected Successfully to database: {}",dsn);
        let dbInfoBuf = core::ptr::null_mut();
        let outlen:*mut SQLSMALLINT = core::ptr::null_mut();
        let supported:*mut SQLUSMALLINT = core::ptr::null_mut();
        /* check to see if SQLGetInfo() is supported */
        SQLGetFunctions(hdbc, SQL_API_SQLGETINFO as u16,
                                supported);
        if supported == SQL_TRUE as *mut u16
        {
            SQLGetInfo(hdbc, SQL_DATA_SOURCE_NAME as u16,
                       dbInfoBuf, 255, outlen);
            let cstr = CStr::from_ptr(dbInfoBuf as *const _).to_string_lossy();
            println!("dbInfoBuf has value: {}", !dbInfoBuf.is_null());
            println!("DSN name: {}", cstr);
        }
        println!("Disconnecting database: {}",dsn);
        SQLDisconnect(hdbc);
        println!("Disconnected Successfully from database: {}",dsn);
        println!("Freeing connection Handle");
        SQLFreeHandle(SQL_HANDLE_DBC as i16,hdbc);
    }

    /*println!("c_string's length is {}", count);
    println!("c_string's length is {:?}", safe_mystrlen("C From Rust"));*/
}
