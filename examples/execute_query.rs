extern crate odbc_safe;

use ibm_db::{ODBCConnectionManager, Statement, ResultSetState::{Data,NoData}, Connection};
use odbc_safe::AutocommitOn;
use std::io;
use std::error::Error;

fn main() {



    match connect() {
        Ok(()) => println!("Success"),
        Err(diag) => println!("Error: {}", diag),
    }
}

fn connect() -> Result<(), Box<dyn Error>> {
    // TODO: Update with your actual database connection details
    let manager = ODBCConnectionManager::new("DRIVER={IBM DB2 ODBC DRIVER};DATABASE=SAMPLE;HOSTNAME=db.example.com;PORT=50000;UID=db_user;PWD=YourPassword123");
    let pool = r2d2::Pool::new(manager).unwrap();
    let pool = pool.clone();
    let pool_conn = pool.get().unwrap();
    let conn = pool_conn.raw();
    execute_statement(&conn)
}

fn execute_statement(conn: &Connection<AutocommitOn>) -> Result<(), Box<dyn Error>> {
    let stmt = Statement::with_parent(conn)?;

    let mut sql_text = String::new();
    println!("Please enter SQL statement string: ");
    io::stdin().read_line(&mut sql_text).unwrap();

    match stmt.exec_direct(&sql_text)? {
        Data(mut stmt) => {
            let cols = stmt.num_result_cols()?;
            while let Some(mut cursor) = stmt.fetch()? {
                for i in 1..(cols + 1) {
                    match cursor.get_data::<&str>(i as u16)? {
                        Some(val) => print!(" {}", val),
                        None => print!(" NULL"),
                    }
                }
                println!();
            }
        }
        NoData(_) => println!("Query executed, no data returned"),
    }

    Ok(())
}