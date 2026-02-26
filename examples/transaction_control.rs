/// Test Case: Transaction Control (Commit and Rollback)
/// ======================================================
/// This example demonstrates:
/// - Disabling autocommit mode
/// - Executing INSERT operations within a transaction
/// - Committing and rolling back transactions
/// - Re-enabling autocommit mode
///
/// Requirements:
/// - Connected to a DB2 database
/// - Table should exist: CREATE TABLE test_transactions (id INT, name VARCHAR(50))

use ibm_db::{
    create_environment_v3,
    Statement,
    ResultSetState::{Data, NoData},
};
use std::error::Error;

fn main() {
    match test_transactions() {
        Ok(()) => println!("✓ Transaction test completed successfully"),
        Err(e) => println!("✗ Transaction test failed: {}", e),
    }
}

fn test_transactions() -> Result<(), Box<dyn Error>> {
    println!("\n========== TRANSACTION CONTROL TEST ==========\n");

    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    // Connect to database
    // TODO: Update with your actual database connection details
    let connection_string = "DRIVER={IBM DB2 ODBC DRIVER};DATABASE=SAMPLE;HOSTNAME=db.example.com;PORT=50000;UID=db_user;PWD=YourPassword123";
    let mut conn = env.connect_with_connection_string(connection_string)?;

    // Disable autocommit to enable transaction control
    let mut conn = match conn.disable_autocommit() {
        Ok(c) => c,
        Err(_) => {
            println!("✗ Failed to disable autocommit mode");
            return Err("Could not disable autocommit".into());
        }
    };

    println!("✓ Autocommit disabled - Transaction mode active\n");

    // Create test table
    let stmt = Statement::with_parent(&conn)?;
    let create_table_sql = "CREATE TABLE IF NOT EXISTS test_tx (id INT, name VARCHAR(50))";

    match stmt.exec_direct(create_table_sql)? {
        Data(s) => {
            let _ = s.close_cursor()?;
            println!("✓ Test table created or already exists");
        }
        NoData(_) => println!("✓ Test table created or already exists"),
    }

    // Test 1: Successful commit
    println!("\n--- Test 1: Successful Commit ---");
    let stmt = Statement::with_parent(&conn)?;
    let insert_sql = "INSERT INTO test_tx (id, name) VALUES (1, 'Commit Test')";

    match stmt.exec_direct(insert_sql)? {
        Data(s) => {
            let _ = s.close_cursor()?;
        }
        NoData(_) => {}
    }

    match conn.commit() {
        Ok(()) => println!("✓ Transaction committed successfully"),
        Err(e) => {
            println!("✗ Commit failed: {}", e);
            let _ = conn.rollback();
        }
    }

    // Test 2: Rollback transaction
    println!("\n--- Test 2: Rollback Transaction ---");
    let stmt = Statement::with_parent(&conn)?;
    let insert_sql = "INSERT INTO test_tx (id, name) VALUES (2, 'Rollback Test')";

    match stmt.exec_direct(insert_sql)? {
        Data(s) => {
            let _ = s.close_cursor()?;
        }
        NoData(_) => {}
    }

    println!("! Inserted row (will be rolled back)");

    match conn.rollback() {
        Ok(()) => println!("✓ Transaction rolled back successfully"),
        Err(e) => println!("✗ Rollback failed: {}", e),
    }

    // Verify rollback worked - row 2 should not exist
    let stmt = Statement::with_parent(&conn)?;
    let verify_sql = "SELECT COUNT(*) FROM test_tx WHERE id = 2";

    match stmt.exec_direct(verify_sql)? {
        Data(mut stmt) => {
            if let Some(mut cursor) = stmt.fetch()? {
                if let Some(count) = cursor.get_data::<i32>(1)? {
                    if count == 0 {
                        println!("✓ Rollback verified: Row not found in database");
                    } else {
                        println!("✗ Rollback failed: Row still exists");
                    }
                }
            }
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Query returned no data"),
    }

    // Re-enable autocommit
    match conn.enable_autocommit() {
        Ok(_) => println!("\n✓ Autocommit re-enabled"),
        Err(_) => println!("\n✗ Failed to re-enable autocommit"),
    }

    println!("\n========== TEST COMPLETE ==========\n");
    Ok(())
}
