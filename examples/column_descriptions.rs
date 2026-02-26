use ibm_db::{Statement, create_environment_v3,ResultSetState::Data};
use std::error::Error;

fn main() {
    test_me().unwrap()
}

fn test_me() -> std::result::Result<(), Box<dyn Error>> {
    let env = create_environment_v3().map_err(|e| {
        e.expect("Can't create ODBC environment")
    })?;
    let connection_string = "DRIVER={IBM DB2 ODBC DRIVER};DATABASE=SAMPLE;HOSTNAME=db.example.com;PORT=50000;UID=db_user;PWD=YourPassword123";
    let conn = env.connect_with_connection_string(connection_string)?;
    let result = Statement::with_parent(&conn)?.exec_direct(
        "select 1,2,3,4,5 from sysibm.sysdummy1",
    )?;

    if let Data(stmt) = result {
        for i in 1..5 {
            let val = stmt.describe_col(i)?;
            println!("Column {}: {:?}", i, val)
        }
    };

    Ok(())
}
