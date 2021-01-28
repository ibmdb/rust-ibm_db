use ibm_db::{Statement, create_environment_v3,ResultSetState::Data};
use std::error::Error;

fn main() {
    test_me().unwrap()
}

fn test_me() -> std::result::Result<(), Box<dyn Error>> {
    let env = create_environment_v3().map_err(|e| {
        e.expect("Can't create ODBC environment")
    })?;
    let conn = env.connect("dashdb", "admin", "admin")?;
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
