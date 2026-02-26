
use std::io;
use ibm_db::{safe::AutocommitOn,Statement,create_environment_v3, Connection,ResultSetState::{NoData, Data}};
use std::error::Error;

fn main() {

    match connect() {
        Ok(()) => println!("Success"),
        Err(diag) => println!("Error: {}", diag),
    }
}

fn connect() -> Result<(), Box<dyn Error>> {

    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    // Direct connection string (no DSN required)
    // Update HOSTNAME, PORT, and DATABASE with your actual connection details
    // TODO: Update with your actual database connection details
    let connection_string = "DRIVER={IBM DB2 ODBC DRIVER};DATABASE=SAMPLE;HOSTNAME=db.example.com;PORT=50000;UID=db_user;PWD=YourPassword123";
    
    let conn = env.connect_with_connection_string(connection_string)?;
    execute_statement(&conn)
}

fn execute_statement<'env>(conn: &Connection<'env, AutocommitOn>) -> Result<(),Box<dyn Error>> {
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
