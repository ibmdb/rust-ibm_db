/// Test Case: Error Handling and Diagnostics
/// ==========================================
/// This example demonstrates:
/// - Handling connection errors gracefully
/// - Managing query execution errors
/// - Diagnostic information from failed operations
/// - Error recovery patterns
/// - Input validation
///
/// Requirements:
/// - Connected to a DB2 database (or attempt connection)

use ibm_db::{
    create_environment_v3,
    Statement,
    ResultSetState::{Data, NoData},
};
use std::error::Error;

fn main() {
    match test_error_handling() {
        Ok(()) => println!("\n✓ Error handling test completed successfully"),
        Err(e) => println!("\n✗ Error handling test failed: {}", e),
    }
}

fn test_error_handling() -> Result<(), Box<dyn Error>> {
    println!("\n========== ERROR HANDLING TEST ==========\n");

    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    // TODO: Update with your actual database connection details
    let connection_string = "DRIVER={IBM DB2 ODBC DRIVER};DATABASE=SAMPLE;HOSTNAME=db.example.com;PORT=50000;UID=db_user;PWD=YourPassword123";
    let conn = env.connect_with_connection_string(connection_string)?;

    println!("✓ Connected to database\n");

    // Test 1: Handle invalid SQL syntax
    println!("--- Test 1: Invalid SQL Syntax ---");
    let stmt = Statement::with_parent(&conn)?;
    let invalid_sql = "SELET * FROM test_types";  // Typo: SELET instead of SELECT

    match stmt.exec_direct(invalid_sql) {
        Ok(_) => println!("✗ Query unexpectedly succeeded"),
        Err(e) => {
            println!("✓ Caught error (as expected)");
            println!("  Error message: {}\n", e);
        }
    }

    // Test 2: Handle non-existent table
    println!("--- Test 2: Non-existent Table ---");
    let stmt = Statement::with_parent(&conn)?;
    let invalid_sql = "SELECT * FROM table_that_does_not_exist_12345";

    match stmt.exec_direct(invalid_sql) {
        Ok(_) => println!("✗ Query unexpectedly succeeded"),
        Err(e) => {
            println!("✓ Caught error (as expected)");
            println!("  Error message: {}\n", e);
        }
    }

    // Test 3: Handle type mismatches
    println!("--- Test 3: Type Mismatch in Comparison ---");
    let stmt = Statement::with_parent(&conn)?;
    let setup = "CREATE TABLE IF NOT EXISTS test_types (id INT)";

    match stmt.exec_direct(setup) {
        Ok(Data(s)) => { let _ = s.close_cursor()?; }
        Ok(NoData(_)) => {}
        Err(e) => println!("Setup error: {}", e),
    }

    let stmt = Statement::with_parent(&conn)?;
    // This might fail depending on DB2 strictness
    match stmt.exec_direct("INSERT INTO test_types VALUES ('not_a_number')") {
        Ok(_) => println!("! Insert succeeded (depends on DB2 behavior)"),
        Err(e) => {
            println!("✓ Caught type mismatch error");
            println!("  Error message: {}\n", e);
        }
    }

    // Test 4: Safe NULL handling
    println!("--- Test 4: Safe NULL Value Handling ---");
    let stmt = Statement::with_parent(&conn)?;

    match stmt.exec_direct("SELECT * FROM test_types WHERE id IS NULL") {
        Ok(Data(mut stmt)) => {
            let count = 0;
            while let Ok(Some(_)) = stmt.fetch() {
                // Handle NULL safely
            }
            let _ = stmt.close_cursor()?;
            println!("✓ NULL values handled safely");
        }
        Ok(NoData(_)) => println!("✓ NULL query executed (no results)"),
        Err(e) => println!("✗ Query failed: {}", e),
    }

    // Test 5: Division by zero handling
    println!("\n--- Test 5: Division by Zero ---");
    let stmt = Statement::with_parent(&conn)?;
    let div_zero = "SELECT 100 / 0 as result";

    match stmt.exec_direct(div_zero) {
        Ok(Data(mut stmt)) => {
            if let Ok(Some(mut cursor)) = stmt.fetch() {
                if let Ok(Some(result)) = cursor.get_data::<f64>(1) {
                    println!("! Result: {} (DB2 returned a value)", result);
                } else {
                    println!("✓ Division by zero returned NULL");
                }
            }
            let _ = stmt.close_cursor()?;
        }
        Ok(NoData(_)) => println!("✓ Division by zero handled (no data)"),
        Err(e) => {
            println!("✓ Division by zero caught as error");
            println!("  Error message: {}", e);
        }
    }

    // Test 6: Parameter binding validation
    println!("\n--- Test 6: Parameter Binding ---");
    let stmt = Statement::with_parent(&conn)?;

    match stmt.prepare("SELECT * FROM test_types WHERE id = ?") {
        Ok(prepared) => {
            // Bind valid parameter
            match prepared.bind_parameter(1, &123i32) {
                Ok(prepared) => {
                    match prepared.execute() {
                        Ok(_) => println!("✓ Parameter bound and executed successfully"),
                        Err(e) => println!("✗ Execution failed: {}", e),
                    }
                }
                Err(e) => println!("✗ Parameter binding failed: {}", e),
            }
        }
        Err(e) => println!("✗ Prepare failed: {}", e),
    }

    // Test 7: Graceful recovery from errors
    println!("\n--- Test 7: Error Recovery ---");
    println!("Attempting multiple operations with error recovery:\n");

    let operations = vec![
        ("SELECT COUNT(*) FROM test_types", "Valid query"),
        ("SELECT * FROM invalid_table", "Invalid table"),
        ("SELECT * FROM test_types WHERE id = 1", "Valid query after error"),
    ];

    let mut success_count = 0;
    let mut error_count = 0;

    for (sql, description) in operations {
        print!("  {} ... ", description);
        let stmt = Statement::with_parent(&conn)?;

        match stmt.exec_direct(sql) {
            Ok(_) => {
                println!("OK");
                success_count += 1;
            }
            Err(_) => {
                println!("ERROR (recovered)");
                error_count += 1;
            }
        }
    }

    println!("\n  Summary: {} successful, {} errors (recovered)", success_count, error_count);
    println!("✓ Error recovery demonstrated\n");

    println!("========== TEST COMPLETE ==========\n");
    Ok(())
}
