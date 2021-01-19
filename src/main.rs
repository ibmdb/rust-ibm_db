#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, improper_ctypes)]

use ::ibm_db::SQLConnect;
use ibm_db::{SQLAllocHandle, SQL_HANDLE_DBC, SQL_HANDLE_ENV, SQL_SUCCESS, SQLFreeHandle, SQLSMALLINT, SQLExecDirect, SQL_NTS, SQLCHAR, SQLSetConnectAttr, SQL_ATTR_AUTOCOMMIT, SQL_AUTOCOMMIT_ON, SQLPOINTER, SQL_HANDLE_STMT, SQL_NO_DATA_FOUND, SQLFreeStmt, SQL_UNBIND, SQL_RESET_PARAMS, SQL_CLOSE};

fn main() {
    let dsn = "dashdb";
    let uid = "admin";
    let pwd = "admin";
    let conn = connect(
        dsn.parse().unwrap(),
        uid.parse().unwrap(),
        pwd.parse().unwrap()
    );
    unsafe{
        if conn == 0{
            println!("Connection failed...");
            std::process::exit(-1);
        }
        dml(conn);
        closeConnection(conn);
    }
}

fn connect(mut dsn : String, mut uid: String, mut pwd: String) -> i32 {
    unsafe {
        let mut hdbc = 0;
        let mut cliRC;
        let mut out = 0;
        cliRC = SQLAllocHandle(SQL_HANDLE_ENV as i16,
                               0,
                               &mut out);
        if cliRC != SQL_SUCCESS as i16 {
            println!("--ERROR while allocating the environment handle. Status: {}", cliRC);
            return 0;
        }

        cliRC = SQLAllocHandle(SQL_HANDLE_DBC as i16,
                               out,
                               &mut hdbc);
        if cliRC != SQL_SUCCESS as i16 {
            println!("--ERROR while getting hdbc. Status: {}", cliRC);
            return 0;
        }
        println!("Connecting to database: {} .........", dsn);
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

        SQLSetConnectAttr(hdbc, SQL_ATTR_AUTOCOMMIT as i32, SQL_AUTOCOMMIT_ON as SQLPOINTER, SQL_NTS);

        let mut hstmt = 0;

        cliRC = SQLAllocHandle(SQL_HANDLE_STMT as SQLSMALLINT,
                               hdbc,
                               &mut hstmt);
        if cliRC != SQL_SUCCESS as i16 {
            println!("--ERROR while getting statement. Status: {}", cliRC);
            return 0;
        }
        println!("Connected Successfully to database: {}", dsn);
        return hstmt;
    }
}

pub unsafe fn dml(conn: i32){
    let mut cliRC;

    println!("Dropping table if it exists.....");
    let mut query = "DROP TABLE TEST";
    let mut stmt = query.as_bytes().as_ptr() as *mut SQLCHAR;
    cliRC = SQLExecDirect(conn, stmt, query.as_bytes().len() as i32);
    println!("Dropping Table Result: {}",cliRC);

    println!("Creating table.....");
    query = "create table test(Col3 VARCHAR(7))";
    stmt = query.as_bytes().as_ptr() as *mut SQLCHAR;
    cliRC = SQLExecDirect(conn, stmt, query.as_bytes().len() as i32);
    println!("Creating Table Result: {}",cliRC);

    println!("Inserting Data.....");
    query = "INSERT INTO TEST VALUES ('Binit')";
    stmt = query.as_bytes().as_ptr() as *mut SQLCHAR;

    cliRC = SQLExecDirect(conn, stmt, query.as_bytes().len() as i32);
    println!("Inserting Data Result: {}",cliRC);

    println!("Fetching Data.....");
    query = "SELECT * FROM TEST";
    stmt = query.as_bytes().as_ptr() as *mut SQLCHAR;
    cliRC = SQLExecDirect(conn, stmt, query.as_bytes().len() as i32);
    if cliRC == SQL_NO_DATA_FOUND as i16{
        println!("No Data in Table");
    } else{
        println!("Fetching Data Result: {}",cliRC);
    }
}
pub unsafe fn closeConnection(conn:i32){
    println!("Disconnecting ...");
    SQLFreeHandle(SQL_HANDLE_STMT as i16, conn);
    SQLFreeStmt(conn, SQL_UNBIND as u16);
    SQLFreeStmt(conn, SQL_RESET_PARAMS as u16);
    SQLFreeStmt(conn, SQL_CLOSE as u16);
    println!("Disconnected Successfully.");

}
