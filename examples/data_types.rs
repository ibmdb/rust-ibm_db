/// Test Case: Data Type Handling
/// =============================
/// This example demonstrates:
/// - Inserting and retrieving various data types
/// - Handling NULL values
/// - Type conversions in SQL queries
/// - Working with strings, integers, floats, and dates
///
/// Requirements:
/// - Connected to a DB2 database
/// - Table should exist with various column types

use ibm_db::{
    create_environment_v3,
    Statement,
    ResultSetState::{Data, NoData},
};
use std::error::Error;

fn main() {
    match test_data_types() {
        Ok(()) => println!("\n✓ Data type test completed successfully"),
        Err(e) => println!("\n✗ Data type test failed: {}", e),
    }
}

fn test_data_types() -> Result<(), Box<dyn Error>> {
    println!("\n========== DATA TYPE HANDLING TEST ==========\n");

    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    // TODO: Update with your actual database connection details
    let connection_string = "DRIVER={IBM DB2 ODBC DRIVER};DATABASE=SAMPLE;HOSTNAME=db.example.com;PORT=50000;UID=db_user;PWD=YourPassword123";
    let conn = env.connect_with_connection_string(connection_string)?;

    println!("✓ Connected to database\n");

    // Create test table with various data types
    let stmt = Statement::with_parent(&conn)?;
    let create_sql = "CREATE TABLE IF NOT EXISTS test_types (
        id INT,
        name VARCHAR(50),
        description VARCHAR(255),
        salary DECIMAL(10,2),
        is_active INT,
        hire_date DATE
    )";

    match stmt.exec_direct(create_sql)? {
        Data(s) => { let _ = s.close_cursor()?; }
        NoData(_) => {}
    }
    println!("✓ Test table created\n");

    // Test 1: Insert various data types
    println!("--- Test 1: Insert Various Data Types ---");

    let test_records = vec![
        (1, "John Doe", "Senior Developer", 85000.00, 1),
        (2, "Jane Smith", "Project Manager", 78500.50, 1),
        (3, "Bob Johnson", "Intern", 35000.00, 0),
        (4, "Alice Williams", "QA Engineer", 62000.75, 1),
    ];

    for (id, name, desc, salary, active) in test_records {
        let sql = format!(
            "INSERT INTO test_types (id, name, description, salary, is_active, hire_date) VALUES ({}, '{}', '{}', {}, {}, CURRENT_DATE)",
            id, name, desc, salary, active
        );

        let stmt = Statement::with_parent(&conn)?;
        match stmt.exec_direct(&sql)? {
            Data(s) => { let _ = s.close_cursor()?; }
            NoData(_) => {}
        }
        println!("✓ Inserted: ID={}, Name='{}', Salary={}", id, name, salary);
    }

    // Test 2: Retrieve and display various types
    println!("\n--- Test 2: Retrieve Different Data Types ---");
    let stmt = Statement::with_parent(&conn)?;

    match stmt.exec_direct("SELECT id, name, salary, is_active FROM test_types WHERE is_active = 1")? {
        Data(mut stmt) => {
            println!("\nActive Employees:");
            println!("{:<5} {:<15} {:<12} {:<8}", "ID", "Name", "Salary", "Active");
            println!("{}", "-".repeat(45));

            while let Some(mut cursor) = stmt.fetch()? {
                let id: Option<i32> = cursor.get_data(1)?;
                let name: Option<String> = cursor.get_data(2)?;
                let salary: Option<f64> = cursor.get_data(3)?;
                let is_active: Option<i32> = cursor.get_data(4)?;

                if let (Some(id), Some(name), Some(salary), Some(active)) = (id, name, salary, is_active) {
                    println!("{:<5} {:<15} ${:<11.2} {}", id, name, salary, active);
                }
            }
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Query returned no data"),
    }

    // Test 3: NULL value handling
    println!("\n--- Test 3: Insert and Handle NULL Values ---");
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("INSERT INTO test_types (id, name) VALUES (5, 'Unknown Employee')")? {
        Data(s) => { let _ = s.close_cursor()?; }
        NoData(_) => {}
    }
    println!("✓ Inserted record with NULL values\n");

    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("SELECT id, name, salary, description FROM test_types WHERE id = 5")? {
        Data(mut stmt) => {
            while let Some(mut cursor) = stmt.fetch()? {
                let id: Option<i32> = cursor.get_data(1)?;
                let name: Option<String> = cursor.get_data(2)?;
                let salary: Option<f64> = cursor.get_data(3)?;
                let desc: Option<String> = cursor.get_data(4)?;

                println!("Retrieved record:");
                println!("  ID: {:?}", id);
                println!("  Name: {:?}", name);
                println!("  Salary: {:?} (NULL)", salary);
                println!("  Description: {:?} (NULL)", desc);
            }
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Query returned no data"),
    }

    // Test 4: Type conversion and functions
    println!("\n--- Test 4: SQL Type Conversion ---");
    let stmt = Statement::with_parent(&conn)?;

    match stmt.exec_direct("SELECT name, CAST(salary AS INT) as salary_int, UPPER(name) as upper_name FROM test_types WHERE id <= 2")? {
        Data(mut stmt) => {
            println!("\nType Conversions:");
            println!("{:<15} {:<12} {:<15}", "Name", "Salary(INT)", "UpperCase");
            println!("{}", "-".repeat(45));

            while let Some(mut cursor) = stmt.fetch()? {
                let name: Option<String> = cursor.get_data(1)?;
                let salary_int: Option<i32> = cursor.get_data(2)?;
                let upper: Option<String> = cursor.get_data(3)?;

                if let (Some(name), Some(salary), Some(upper)) = (name, salary_int, upper) {
                    println!("{:<15} {:<12} {:<15}", name, salary, upper);
                }
            }
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Query returned no data"),
    }

    // Test 5: Aggregate functions with different types
    println!("\n--- Test 5: Aggregate Functions ---");
    let stmt = Statement::with_parent(&conn)?;

    match stmt.exec_direct("SELECT COUNT(*) as cnt, AVG(salary) as avg_sal, MAX(salary) as max_sal, MIN(salary) as min_sal FROM test_types")? {
        Data(mut stmt) => {
            if let Some(mut cursor) = stmt.fetch()? {
                let count: Option<i32> = cursor.get_data(1)?;
                let avg: Option<f64> = cursor.get_data(2)?;
                let max: Option<f64> = cursor.get_data(3)?;
                let min: Option<f64> = cursor.get_data(4)?;

                println!("\nAggregate Statistics:");
                println!("  Total Records: {}", count.unwrap_or(0));
                if let Some(avg_val) = avg {
                    println!("  Average Salary: ${:.2}", avg_val);
                }
                if let Some(max_val) = max {
                    println!("  Maximum Salary: ${:.2}", max_val);
                }
                if let Some(min_val) = min {
                    println!("  Minimum Salary: ${:.2}", min_val);
                }
            }
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Query returned no data"),
    }

    println!("\n========== TEST COMPLETE ==========\n");
    Ok(())
}
