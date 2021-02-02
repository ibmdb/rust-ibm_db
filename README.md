# rust_ibm_db(Beta)

Interface for Rust to DB2 for z/OS, DB2 for LUW, DB2 for i.

## API Documentation

> For complete list of rust_ibm_db APIs refer to ODBC documentation

## Prerequisite

> RUST should be installed(Rust version should be >=1.45)
Confirm by typing below in command prompt:

```
>rustc --version

```

> We'll assume that you've installed Git, forked [rust-ibm_db](https://github.com/ibmdb/rust-ibm_db.git), and cloned the forked repo to your PC.
>
> We'll use the command line interface to interact with Git i.e.
```
git clone https://github.com/ibmdb/rust-ibm_db
```
> 
> There are also a number of GUIs and IDE integrations that can generally do the same things.
> 
> If you've cloned your fork, then you will be able to reference it with origin in your local repo. 

> CLI Driver should be downloaded in your system and IBM_DB_PATH, LD_LIBRARY_PATH should be set to point to CLI Driver folder.
> 
> If CLI Driver is not installed run the below command once you checkout the GIT repo and it will be installed:
```
cargo run --package ibm_db --example setup
```

> NOTE: In order for the test program to run, DSN needs to be configured. Update the db2dsdriver.cfg file(present in /clidriver/cfg folder under CLI driver path) with the requisite details.
```
e.g.
<?xml version="1.0" encoding="UTF-8" standalone="no" ?>
<configuration>
  <dsncollection>
	<dsn alias="dashdb4" host="test@test.com" name="FOO" port="0000"/>
	</dsncollection>

  <databases>
	<database host="test@test.com" name="FOO" port="0000"/>
	</databases>

</configuration>
```

## How to Install
```
Include ibm_db in your cargo.toml with latest version.
Crates.io link: https://crates.io/crates/ibm_db

OR simply include this project in your RUST project.

If you already have a cli driver available in your system, add the path of the same to your IBM_DB_HOME, PATH and LD_LIBRARY_PATH
Example: set/export PATH = /IBM/IBM DATA SERVER DRIVER/bin


If you do not have a clidriver in your system, go to examples folder where rust_ibm_db is downloaded in your system (Example: /Users/uname/rust_ibm_db/installer) and run setup.rs file: 
> cargo run --package ibm_db --example setup

where uname is the username

Above command will download clidriver.

Add the path of the clidriver downloaded to your Path windows/LINUX/MACOS environment variable i.e. IBM_DB_HOME, PATH and LD_LIBRARY_PATH or DYLD_LIBRARY_PATH depending on Windows/LINUX/MACOS
(Example: set/export PATH = /IBM/IBM DATA SERVER DRIVER/clidriver/bin)


```

### <a name="Licenserequirements"></a> License requirements for connecting to databases

rust_ibm_db driver can connect to DB2 on Linux Unix and Windows without any additional license/s, however, connecting to databases on DB2 for z/OS or DB2 for i(AS400) Servers require either client side or server side license/s. The client side license would need to be copied under `license` folder of your `clidriver` installation directory and for activating server side license, you would need to purchase DB2 Connect Unlimited for System z® and DB2 Connect Unlimited Edition for System i®.

To know more about license and purchasing cost, please contact [IBM Customer Support](http://www-05.ibm.com/support/operations/zz/en/selectcountrylang.html).

To know more about server based licensing viz db2connectactivate, follow below links:
* [Activating the license certificate file for DB2 Connect Unlimited Edition](https://www.ibm.com/developerworks/community/blogs/96960515-2ea1-4391-8170-b0515d08e4da/entry/unlimited_licensing_in_non_java_drivers_using_db2connectactivate_utlility1?lang=en).
* [Unlimited licensing using db2connectactivate utility](https://www.ibm.com/developerworks/community/blogs/96960515-2ea1-4391-8170-b0515d08e4da/entry/unlimited_licensing_in_non_java_drivers_using_db2connectactivate_utlility1?lang=en.)

## How to run sample program

### main.rs

```


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
        let mut hdbc = 0;
        let mut cliRC;
        let mut out = 0;
        cliRC = SQLAllocHandle(SQL_HANDLE_ENV as i16,
                               0,
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

        let mut hstmt= 0;

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


```
To run the sample:- 

```

cargo run

```

## NOTE for MACOS:
If you get an error i.e. "dyld: Library not loaded: libdb2.dylib"
Run the following command(Where replace the <RUST_CRATE_LIB> with the path of your rust program root folder):

```
install_name_tool -change libdb2.dylib $IBM_DB_HOME/lib/libdb2.dylib <RUST_CRATE_LIB>/target/debug/ibm_db

```