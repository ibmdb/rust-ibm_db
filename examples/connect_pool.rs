extern crate r2d2;

use std::thread;
use ibm_db::{ODBCConnectionManager,Statement,ResultSetState::Data};

fn main() {
    let manager = ODBCConnectionManager::new("DSN=dashdb;DATABASE=FOO;hostname=test@test.com;PORT=0000;UID=admin;PWD=admin;");
    let pool = r2d2::Pool::new(manager).unwrap();

    let mut children = vec![];
    for i in 0..10i32 {
        let pool = pool.clone();
        let pool_conn = pool.get().unwrap();
        children.push(thread::spawn(move || {
            let conn = pool_conn.raw();
            let stmt = Statement::with_parent(&conn).unwrap();
            if let Data(mut stmt) = stmt.exec_direct("select * from dbtest.GLWTACT").unwrap() {
                while let Some(mut cursor) = stmt.fetch().unwrap() {
                    if let Some(val) = cursor.get_data::<&str>(1).unwrap() {
                        println!("THREAD {} {}", i, val);
                    }
                }
            }
        }));
    }

    for child in children {
        let _ = child.join();
    }
}