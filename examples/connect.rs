
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

    //Method-1 for Connection
    /*let mut buffer = String::new();
    println!("Please enter connection string: ");
    io::stdin().read_line(&mut buffer).unwrap();

    let conn = env.connect_with_connection_string(&buffer)?;*/

    //Method-2 for Connection
    let conn = env.connect("dashdb", "admin", "admin").unwrap();
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
