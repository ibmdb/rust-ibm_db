#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, improper_ctypes)]

use ::CCODE::SQLConnect;
use CCODE::{SQLAllocHandle, SQL_HANDLE_DBC, SQL_HANDLE_ENV, SQL_SUCCESS, SQLDisconnect, SQLFreeHandle, SQLGetInfo, SQLSMALLINT, SQL_DATA_SOURCE_NAME, SQLGetFunctions, SQL_API_SQLGETINFO, SQLUSMALLINT, SQL_TRUE, SQLExecDirect, SQL_NTS, SQLCHAR, SQLSetConnectAttr, SQL_ATTR_AUTOCOMMIT, SQL_AUTOCOMMIT_ON, SQLPOINTER, SQL_HANDLE_STMT, SQL_NO_DATA_FOUND, SQLFreeStmt, SQL_UNBIND, SQL_RESET_PARAMS, SQL_CLOSE};
use std::ffi::CStr;


fn main() {
    connect(
        "dashdb".parse().unwrap(),
        "admin".parse().unwrap(),
        "admin".parse().unwrap()
    )
}

fn connect(mut dsn : String, mut uid: String, mut pwd: String){

    

    unsafe{
        let mut hdbc= core::ptr::null_mut();
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

        let mut hstmt= core::ptr::null_mut();

        SQLSetConnectAttr(hdbc,SQL_ATTR_AUTOCOMMIT as i32,SQL_AUTOCOMMIT_ON as SQLPOINTER,SQL_NTS);

        println!("Allocating Statement Handle");
        cliRC = SQLAllocHandle(SQL_HANDLE_STMT as SQLSMALLINT,
                               hdbc,
                               &mut hstmt);
        if cliRC!= SQL_SUCCESS as i16 {
            println!("--ERROR while getting hdbc. Status: {}",cliRC);
            return;
        }

        println!("Dropping table if it exists.....");
        let mut query = "DROP TABLE TEST";
        let mut stmt = query.as_bytes().as_ptr() as *mut SQLCHAR;
        cliRC = SQLExecDirect(hstmt,stmt,query.as_bytes().len() as i32);
        println!("Dropping Table Result: {}",cliRC);

        println!("Creating table.....");
        query = "create table test(Col3 VARCHAR(7))";
        stmt = query.as_bytes().as_ptr() as *mut SQLCHAR;
        cliRC = SQLExecDirect(hstmt,stmt,query.as_bytes().len() as i32);
        println!("Creating Table Result: {}",cliRC);

        println!("Inserting Data.....");
        query = "INSERT INTO TEST VALUES ('Binit')";
        stmt = query.as_bytes().as_ptr() as *mut SQLCHAR;

        cliRC = SQLExecDirect(hstmt,stmt,query.as_bytes().len() as i32);
        println!("Inserting Data Result: {}",cliRC);


        println!("Fetching Data.....");
        query = "SELECT * FROM TEST";
        stmt = query.as_bytes().as_ptr() as *mut SQLCHAR;
        cliRC = SQLExecDirect(hstmt,stmt,query.as_bytes().len() as i32);
        if cliRC == SQL_NO_DATA_FOUND as i16{
            println!("No Data in Table");
        } else{
            println!("Fetching Data Result: {}",cliRC);
        }

        //After Select, Handle is missing so recreating the same.
        println!("Allocating Statement Handle");
        cliRC = SQLAllocHandle(SQL_HANDLE_STMT as SQLSMALLINT,
                               hdbc,
                               &mut hstmt);
        if cliRC!= SQL_SUCCESS as i16 {
            println!("--ERROR while getting hdbc. Status: {}",cliRC);
            return;
        }


        println!("Dropping table.....");
        query = "DROP TABLE TEST";
        stmt = query.as_bytes().as_ptr() as *mut SQLCHAR;
        cliRC = SQLExecDirect(hstmt,stmt,query.as_bytes().len() as i32);
        println!("Dropping Table Result: {}",cliRC);


        println!("Freeing Statement Handle");
        SQLFreeHandle(SQL_HANDLE_STMT as i16,hstmt);
        SQLFreeStmt(hstmt,SQL_UNBIND as u16);
        SQLFreeStmt(hstmt,SQL_RESET_PARAMS as u16);
        SQLFreeStmt(hstmt,SQL_CLOSE as u16);

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
}
