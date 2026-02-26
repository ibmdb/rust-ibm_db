/// Test Case: Advanced Parameter Binding
/// ======================================
/// This example demonstrates:
/// - Binding multiple parameter types (string, integer, floating-point)
/// - Executing prepared statements with bound parameters
/// - Resetting parameters for statement reuse
/// - Multiple executions with different parameter values
///
/// Requirements:
/// - Connected to a DB2 database
/// - Table should exist: CREATE TABLE test_params (id INT, name VARCHAR(50), salary DECIMAL(10,2))

use ibm_db::{
    create_environment_v3,
    Statement,
    ResultSetState::{Data, NoData},
};
use std::error::Error;

fn main() {
    match test_parameter_binding() {
        Ok(()) => println!("\n✓ Parameter binding test completed successfully"),
        Err(e) => println!("\n✗ Parameter binding test failed: {}", e),
    }
}

fn test_parameter_binding() -> Result<(), Box<dyn Error>> {
    println!("\n========== PARAMETER BINDING TEST ==========\n");

    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    // TODO: Update with your actual database connection details
    let connection_string = "DRIVER={IBM DB2 ODBC DRIVER};DATABASE=SAMPLE;HOSTNAME=db.example.com;PORT=50000;UID=db_user;PWD=YourPassword123";
    let conn = env.connect_with_connection_string(connection_string)?;

    println!("✓ Connected to database\n");

    // Create test table
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("CREATE TABLE IF NOT EXISTS test_params (id INT, name VARCHAR(50), salary DECIMAL(10,2))")? {
        Data(s) => { let _ = s.close_cursor()?; }
        NoData(_) => {}
    }
    println!("✓ Test table ready\n");

    // Test 1: Bind integer and string parameters
    println!("--- Test 1: Bind Integer and String ---");
    let stmt = Statement::with_parent(&conn)?;
    let prepared = stmt.prepare("INSERT INTO test_params (id, name, salary) VALUES (?, ?, ?)")?;

    let id = 101i32;
    let name = "Alice Johnson";
    let salary = 75000.50f64;

    let prepared = prepared.bind_parameter(1, &id)?;
    let prepared = prepared.bind_parameter(2, &name)?;
    let prepared = prepared.bind_parameter(3, &salary)?;

    match prepared.execute()? {
        Data(s) => { let _ = s.close_cursor()?; println!("✓ Inserted: ID={}, Name='{}', Salary={}", id, name, salary); }
        NoData(_) => println!("✓ Inserted: ID={}, Name='{}', Salary={}", id, name, salary),
    }

    // Test 2: Reuse statement with different parameters
    println!("\n--- Test 2: Reset and Reuse Statement ---");
    let stmt = Statement::with_parent(&conn)?;
    let prepared = stmt.prepare("INSERT INTO test_params (id, name, salary) VALUES (?, ?, ?)")?;

    let test_data = vec![
        (102i32, "Bob Smith", 68500.75f64),
        (103i32, "Carol White", 82000.00f64),
        (104i32, "David Brown", 71250.50f64),
    ];

    for (id, name, salary) in test_data {
        let prepared = prepared.bind_parameter(1, &id)?;
        let prepared = prepared.bind_parameter(2, &name)?;
        let prepared = prepared.bind_parameter(3, &salary)?;

        match prepared.execute()? {
            Data(s) => { let _ = s.close_cursor()?; }
            NoData(_) => {}
        }

        let prepared = prepared.reset_parameters()?;
        println!("✓ Inserted: ID={}, Name='{}', Salary={}", id, name, salary);
    }

    // Test 3: Query with parameter (SELECT with WHERE clause)
    println!("\n--- Test 3: Parameterized SELECT ---");
    let stmt = Statement::with_parent(&conn)?;
    let prepared = stmt.prepare("SELECT name, salary FROM test_params WHERE id = ?")?;

    let search_id = 102i32;
    let prepared = prepared.bind_parameter(1, &search_id)?;

    match prepared.execute()? {
        Data(mut stmt) => {
            println!("✓ Searching for employee with ID={}", search_id);
            while let Some(mut cursor) = stmt.fetch()? {
                if let Some(name) = cursor.get_data::<String>(1)? {
                    if let Some(salary) = cursor.get_data::<f64>(2)? {
                        println!("  Found: {} - ${:.2}", name, salary);
                    }
                }
            }
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ No employee found with ID={}", search_id),
    }

    // Test 4: Update with parameters
    println!("\n--- Test 4: Parameterized UPDATE ---");
    let stmt = Statement::with_parent(&conn)?;
    let prepared = stmt.prepare("UPDATE test_params SET salary = ? WHERE id = ?")?;

    let new_salary = 85000.00f64;
    let update_id = 101i32;

    let prepared = prepared.bind_parameter(1, &new_salary)?;
    let prepared = prepared.bind_parameter(2, &update_id)?;

    match prepared.execute()? {
        Data(s) => { let _ = s.close_cursor()?; }
        NoData(_) => {}
    }

    println!("✓ Updated salary to ${:.2} for ID={}", new_salary, update_id);

    // Verify update
    let stmt = Statement::with_parent(&conn)?;
    let prepared = stmt.prepare("SELECT salary FROM test_params WHERE id = ?")?;
    let prepared = prepared.bind_parameter(1, &update_id)?;

    match prepared.execute()? {
        Data(mut stmt) => {
            if let Some(mut cursor) = stmt.fetch()? {
                if let Some(salary) = cursor.get_data::<f64>(1)? {
                    println!("✓ Verified: New salary = ${:.2}", salary);
                }
            }
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Could not verify update"),
    }

    println!("\n========== TEST COMPLETE ==========\n");
    Ok(())
}
