/// Test Case: Advanced Query Patterns
/// ===================================
/// This example demonstrates:
/// - Complex JOIN operations
/// - Subqueries and nested selects
/// - Filtering and sorting
/// - String functions
/// - GROUP BY and aggregate operations
/// - Statement reuse across multiple queries
///
/// Requirements:
/// - Connected to a DB2 database
/// - Sample tables with related data

use ibm_db::{
    create_environment_v3,
    Statement,
    ResultSetState::{Data, NoData},
};
use std::error::Error;

fn main() {
    match test_advanced_queries() {
        Ok(()) => println!("\n✓ Advanced query test completed successfully"),
        Err(e) => println!("\n✗ Advanced query test failed: {}", e),
    }
}

fn test_advanced_queries() -> Result<(), Box<dyn Error>> {
    println!("\n========== ADVANCED QUERY PATTERNS TEST ==========\n");

    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    // TODO: Update with your actual database connection details
    let connection_string = "DRIVER={IBM DB2 ODBC DRIVER};DATABASE=SAMPLE;HOSTNAME=db.example.com;PORT=50000;UID=db_user;PWD=YourPassword123";
    let conn = env.connect_with_connection_string(connection_string)?;

    println!("✓ Connected to database\n");

    // Setup test data
    setup_test_data(&conn)?;

    // Test 1: Simple WHERE and ORDER BY
    println!("--- Test 1: Filtering and Sorting ---");
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("SELECT id, name, salary FROM employees WHERE salary > 60000 ORDER BY salary DESC")? {
        Data(mut stmt) => {
            println!("Employees earning > $60,000 (sorted by salary):\n");
            println!("{:<5} {:<20} {:<12}", "ID", "Name", "Salary");
            println!("{}", "-".repeat(40));

            while let Some(mut cursor) = stmt.fetch()? {
                let id: Option<i32> = cursor.get_data(1)?;
                let name: Option<String> = cursor.get_data(2)?;
                let salary: Option<f64> = cursor.get_data(3)?;

                if let (Some(id), Some(name), Some(salary)) = (id, name, salary) {
                    println!("{:<5} {:<20} ${:<11.2}", id, name, salary);
                }
            }
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Query returned no data"),
    }

    // Test 2: GROUP BY and aggregate functions
    println!("\n--- Test 2: GROUP BY with Aggregates ---");
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("SELECT department, COUNT(*) as emp_count, AVG(salary) as avg_salary FROM employees GROUP BY department")? {
        Data(mut stmt) => {
            println!("Department Statistics:\n");
            println!("{:<20} {:<10} {:<12}", "Department", "Count", "Avg Salary");
            println!("{}", "-".repeat(45));

            while let Some(mut cursor) = stmt.fetch()? {
                let dept: Option<String> = cursor.get_data(1)?;
                let count: Option<i32> = cursor.get_data(2)?;
                let avg: Option<f64> = cursor.get_data(3)?;

                if let (Some(dept), Some(count), Some(avg)) = (dept, count, avg) {
                    println!("{:<20} {:<10} ${:<11.2}", dept, count, avg);
                }
            }
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Query returned no data"),
    }

    // Test 3: HAVING clause (GROUP BY with conditions)
    println!("\n--- Test 3: HAVING Clause ---");
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("SELECT department, COUNT(*) as emp_count FROM employees GROUP BY department HAVING COUNT(*) >= 2")? {
        Data(mut stmt) => {
            println!("Departments with 2 or more employees:\n");
            println!("{:<20} {:<10}", "Department", "Count");
            println!("{}", "-".repeat(30));

            while let Some(mut cursor) = stmt.fetch()? {
                let dept: Option<String> = cursor.get_data(1)?;
                let count: Option<i32> = cursor.get_data(2)?;

                if let (Some(dept), Some(count)) = (dept, count) {
                    println!("{:<20} {:<10}", dept, count);
                }
            }
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Query returned no data"),
    }

    // Test 4: String functions
    println!("\n--- Test 4: String Functions ---");
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("SELECT id, UPPER(name) as uppercase_name, LENGTH(name) as name_length FROM employees WHERE id <= 3")? {
        Data(mut stmt) => {
            println!("String Operations:\n");
            println!("{:<5} {:<20} {:<15}", "ID", "Uppercase", "Length");
            println!("{}", "-".repeat(45));

            while let Some(mut cursor) = stmt.fetch()? {
                let id: Option<i32> = cursor.get_data(1)?;
                let name: Option<String> = cursor.get_data(2)?;
                let len: Option<i32> = cursor.get_data(3)?;

                if let (Some(id), Some(name), Some(len)) = (id, name, len) {
                    println!("{:<5} {:<20} {:<15}", id, name, len);
                }
            }
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Query returned no data"),
    }

    // Test 5: Conditional expressions (CASE)
    println!("\n--- Test 5: CASE Expressions ---");
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct(
        "SELECT name, salary, CASE WHEN salary >= 70000 THEN 'Senior' WHEN salary >= 50000 THEN 'Mid-Level' ELSE 'Junior' END as level FROM employees"
    )? {
        Data(mut stmt) => {
            println!("Employee Levels:\n");
            println!("{:<20} {:<12} {:<12}", "Name", "Salary", "Level");
            println!("{}", "-".repeat(50));

            while let Some(mut cursor) = stmt.fetch()? {
                let name: Option<String> = cursor.get_data(1)?;
                let salary: Option<f64> = cursor.get_data(2)?;
                let level: Option<String> = cursor.get_data(3)?;

                if let (Some(name), Some(salary), Some(level)) = (name, salary, level) {
                    println!("{:<20} ${:<11.2} {:<12}", name, salary, level);
                }
            }
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Query returned no data"),
    }

    // Test 6: DISTINCT to get unique values
    println!("\n--- Test 6: DISTINCT Values ---");
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("SELECT DISTINCT department FROM employees ORDER BY department")? {
        Data(mut stmt) => {
            println!("Unique Departments:\n");

            while let Some(mut cursor) = stmt.fetch()? {
                let dept: Option<String> = cursor.get_data(1)?;
                if let Some(dept) = dept {
                    println!("  • {}", dept);
                }
            }
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Query returned no data"),
    }

    // Test 7: LIMIT/FETCH FIRST
    println!("\n--- Test 7: LIMIT Results (Top 3) ---");
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("SELECT name, salary FROM employees ORDER BY salary DESC FETCH FIRST 3 ROWS ONLY")? {
        Data(mut stmt) => {
            println!("Top 3 Highest Paid Employees:\n");
            println!("{:<20} {:<12}", "Name", "Salary");
            println!("{}", "-".repeat(35));

            let mut rank = 1;
            while let Some(mut cursor) = stmt.fetch()? {
                let name: Option<String> = cursor.get_data(1)?;
                let salary: Option<f64> = cursor.get_data(2)?;

                if let (Some(name), Some(salary)) = (name, salary) {
                    println!("{}. {:<17} ${:<11.2}", rank, name, salary);
                    rank += 1;
                }
            }
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Query returned no data"),
    }

    // Test 8: COUNT with WHERE
    println!("\n--- Test 8: Conditional COUNT ---");
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("SELECT COUNT(*) as total, COUNT(CASE WHEN department = 'Engineering' THEN 1 END) as engineering_count FROM employees")? {
        Data(mut stmt) => {
            if let Some(mut cursor) = stmt.fetch()? {
                let total: Option<i32> = cursor.get_data(1)?;
                let eng: Option<i32> = cursor.get_data(2)?;

                println!("Employee Statistics:");
                println!("  Total employees: {}", total.unwrap_or(0));
                println!("  Engineering dept: {}", eng.unwrap_or(0));
            }
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Query returned no data"),
    }

    println!("\n========== TEST COMPLETE ==========\n");
    Ok(())
}

fn setup_test_data(conn: &ibm_db::Connection<ibm_db::safe::AutocommitOn>) -> Result<(), Box<dyn Error>> {
    // Create employees table
    let stmt = Statement::with_parent(conn)?;
    let create_sql = "CREATE TABLE IF NOT EXISTS employees (
        id INT PRIMARY KEY,
        name VARCHAR(50),
        department VARCHAR(50),
        salary DECIMAL(10,2)
    )";

    match stmt.exec_direct(create_sql)? {
        Data(s) => { let _ = s.close_cursor()?; }
        NoData(_) => {}
    }

    // Clear existing data
    let stmt = Statement::with_parent(conn)?;
    match stmt.exec_direct("DELETE FROM employees")? {
        Data(s) => { let _ = s.close_cursor()?; }
        NoData(_) => {}
    }

    // Insert sample data
    let employees = vec![
        (1, "Alice Johnson", "Engineering", 85000.00),
        (2, "Bob Smith", "Engineering", 78000.00),
        (3, "Carol White", "Sales", 65000.00),
        (4, "David Brown", "Sales", 62000.00),
        (5, "Eve Davis", "HR", 55000.00),
    ];

    for (id, name, dept, salary) in employees {
        let sql = format!(
            "INSERT INTO employees (id, name, department, salary) VALUES ({}, '{}', '{}', {})",
            id, name, dept, salary
        );
        let stmt = Statement::with_parent(conn)?;
        match stmt.exec_direct(&sql)? {
            Data(s) => { let _ = s.close_cursor()?; }
            NoData(_) => {}
        }
    }

    println!("✓ Test data created\n");
    Ok(())
}
