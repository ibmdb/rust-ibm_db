/// Test Case: Affected Row Count and Statement Status
/// ====================================================
/// This example demonstrates:
/// - Tracking affected row counts from INSERT, UPDATE, DELETE operations
/// - Querying affected_row_count() after statement execution
/// - Validating operation success through row count
/// - Handling bulk operations
///
/// Requirements:
/// - Connected to a DB2 database
/// - Table should exist: CREATE TABLE test_rows (id INT, name VARCHAR(50))

use ibm_db::{
    create_environment_v3,
    Statement,
    ResultSetState::{Data, NoData},
};
use std::error::Error;

fn main() {
    match test_affected_rows() {
        Ok(()) => println!("\n✓ Affected rows test completed successfully"),
        Err(e) => println!("\n✗ Affected rows test failed: {}", e),
    }
}

fn test_affected_rows() -> Result<(), Box<dyn Error>> {
    println!("\n========== AFFECTED ROW COUNT TEST ==========\n");

    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    // TODO: Update with your actual database connection details
    let connection_string = "DRIVER={IBM DB2 ODBC DRIVER};DATABASE=SAMPLE;HOSTNAME=db.example.com;PORT=50000;UID=db_user;PWD=YourPassword123";
    let conn = env.connect_with_connection_string(connection_string)?;

    println!("✓ Connected to database\n");

    // Setup test table
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("CREATE TABLE IF NOT EXISTS test_rows (id INT PRIMARY KEY, name VARCHAR(50))")? {
        Data(s) => { let _ = s.close_cursor()?; }
        NoData(_) => {}
    }
    println!("✓ Test table ready\n");

    // Clean up previous test data
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("DELETE FROM test_rows")? {
        Data(s) => {
            let rows = s.affected_row_count()?;
            let _ = s.close_cursor()?;
            println!("✓ Cleaned up {} old rows\n", rows);
        }
        NoData(s) => {
            let rows = s.affected_row_count()?;
            println!("✓ Cleaned up {} old rows\n", rows);
        }
    }

    // Test 1: Single INSERT
    println!("--- Test 1: Single INSERT ---");
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("INSERT INTO test_rows (id, name) VALUES (1, 'Alice')")? {
        Data(s) => {
            let rows = s.affected_row_count()?;
            println!("✓ INSERT affected {} row(s)", rows);
            let _ = s.close_cursor()?;
        }
        NoData(s) => {
            let rows = s.affected_row_count()?;
            println!("✓ INSERT affected {} row(s)", rows);
        }
    }

    // Test 2: Multiple INSERTs
    println!("\n--- Test 2: Multiple INSERTs ---");
    let inserts = vec![
        (2, "Bob"),
        (3, "Carol"),
        (4, "David"),
        (5, "Eve"),
    ];

    for (id, name) in inserts {
        let sql = format!("INSERT INTO test_rows (id, name) VALUES ({}, '{}')", id, name);
        let stmt = Statement::with_parent(&conn)?;
        match stmt.exec_direct(&sql)? {
            Data(s) => {
                let rows = s.affected_row_count()?;
                println!("✓ Inserted '{}': affected {} row(s)", name, rows);
                let _ = s.close_cursor()?;
            }
            NoData(s) => {
                let rows = s.affected_row_count()?;
                println!("✓ Inserted '{}': affected {} row(s)", name, rows);
            }
        }
    }

    // Test 3: UPDATE with condition
    println!("\n--- Test 3: UPDATE Operation ---");
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("UPDATE test_rows SET name = 'Robert' WHERE id = 2")? {
        Data(s) => {
            let rows = s.affected_row_count()?;
            println!("✓ UPDATE affected {} row(s)", rows);
            let _ = s.close_cursor()?;
        }
        NoData(s) => {
            let rows = s.affected_row_count()?;
            println!("✓ UPDATE affected {} row(s)", rows);
        }
    }

    // Test 4: UPDATE multiple rows
    println!("\n--- Test 4: UPDATE Multiple Rows ---");
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("UPDATE test_rows SET name = UPPER(name) WHERE id <= 3")? {
        Data(s) => {
            let rows = s.affected_row_count()?;
            println!("✓ UPDATE affected {} row(s)", rows);
            let _ = s.close_cursor()?;
        }
        NoData(s) => {
            let rows = s.affected_row_count()?;
            println!("✓ UPDATE affected {} row(s)", rows);
        }
    }

    // Test 5: DELETE with condition
    println!("\n--- Test 5: DELETE Operation ---");
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("DELETE FROM test_rows WHERE id = 5")? {
        Data(s) => {
            let rows = s.affected_row_count()?;
            println!("✓ DELETE affected {} row(s)", rows);
            let _ = s.close_cursor()?;
        }
        NoData(s) => {
            let rows = s.affected_row_count()?;
            println!("✓ DELETE affected {} row(s)", rows);
        }
    }

    // Test 6: Verify final state
    println!("\n--- Test 6: Verify Final State ---");
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("SELECT COUNT(*) as row_count FROM test_rows")? {
        Data(mut stmt) => {
            if let Some(mut cursor) = stmt.fetch()? {
                if let Some(count) = cursor.get_data::<i32>(1)? {
                    println!("✓ Total rows in table: {}", count);
                }
            }
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Query returned no data"),
    }

    // Test 7: SELECT returns row count but not affected rows
    println!("\n--- Test 7: SELECT Statement ---");
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("SELECT * FROM test_rows")? {
        Data(mut stmt) => {
            let mut row_count = 0;
            while let Some(_) = stmt.fetch()? {
                row_count += 1;
            }
            let affected = stmt.affected_row_count()?;
            println!("✓ SELECT fetched {} rows", row_count);
            println!("✓ Affected rows count: {} (typically 0 for SELECT)", affected);
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Query returned no data"),
    }

    println!("\n========== TEST COMPLETE ==========\n");
    Ok(())
}
