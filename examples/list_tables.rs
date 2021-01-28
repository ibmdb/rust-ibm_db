
extern crate odbc_safe;
#[macro_use]extern crate prettytable;

use prettytable::{Table};
use ibm_db::{safe::AutocommitOn,Statement,create_environment_v3, Connection};
use std::error::Error;

fn main() {

    match connect() {
        Ok(()) => println!("Success"),
        Err(diag) => println!("Error: {}", diag),
    }
}

fn connect() -> Result<(), Box<dyn Error>> {
    let env = create_environment_v3().map_err(|e| e.unwrap())?;
    //let conn = env.connect("dashdb", "db2admin", "gr8tcode!").unwrap();
    let conn = env.connect("dashdb4", "admin", "admin").unwrap();
    list_tables(&conn)
}

fn list_tables(conn: &Connection<AutocommitOn>) -> Result<(), Box<dyn Error>> {
    let stmt = Statement::with_parent(conn)?;
    // Create the table
    let mut table = Table::new();
    // Add a row per time
    table.add_row(row!["CATALOG_NAME","SCHEMA_NAME", "TABLE_NAME","TYPE"]);
    // Print the table to stdout
    table.printstd();

    let mut rs = stmt.tables_str("%", "%", "%", "TABLE")?;
    let cols = rs.num_result_cols()?;
    while let Some(mut cursor) = rs.fetch()? {

        for i in 1..(cols + 1) {
            match cursor.get_data::<&str>(i as u16)? {
                Some(val) => print!(" {},", val),
                None => print!(" NULL,"),
            }
        }
        //table.add_row(row![val_temp[1],val_temp[2],val_temp[3],val_temp[4],val_temp[5]]);
        println!();
    }
    Ok(())
}
