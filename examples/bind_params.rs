use ibm_db::{create_environment_v3, Statement,ResultSetState::Data};
use std::error::Error;

fn main() {
    test_me().unwrap()
}

fn test_me() -> std::result::Result<(), Box<dyn Error>> {
    let env = create_environment_v3().expect("Can't create ODBC environment");
    let connection_string = "DRIVER={IBM DB2 ODBC DRIVER};DATABASE=SAMPLE;HOSTNAME=db.example.com;PORT=50000;UID=db_user;PWD=YourPassword123";
    let conn = env.connect_with_connection_string(connection_string)?;
    let stmt = Statement::with_parent(&conn)?.prepare(
        "select * from test where COL3 = ?",
    )?;

    let param = "Binit";

    let stmt = stmt.bind_parameter(1, &param)?;
    /*let stmt = stmt.bind_parameter(2, &param)?;
    let stmt = stmt.bind_parameter(3, &param)?;
    let stmt = stmt.bind_parameter(4, &param)?;*/

    let stmt = if let Data(mut stmt) = stmt.execute()? {
        if let Some(mut cursor) = stmt.fetch()? {
            println!("{}", cursor.get_data::<String>(1)?.unwrap());
        }
        stmt.close_cursor()?
    } else {
        panic!("SELECT statement returned no result set");
    };

    let stmt = stmt.reset_parameters()?;

    let param = 128u8;

    let stmt = stmt.bind_parameter(1, &param)?;
    let stmt = stmt.bind_parameter(2, &param)?;

    let stmt = if let Data(mut stmt) = stmt.execute()? {
        if let Some(mut cursor) = stmt.fetch()? {
            println!("{}", cursor.get_data::<String>(1)?.unwrap());
        }
        stmt.close_cursor()?
    } else {
        panic!("SELECT statement returned no result set");
    };
    stmt.reset_parameters().unwrap();

    Ok(())
}
