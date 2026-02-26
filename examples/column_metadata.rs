/// Test Case: Column Metadata and Description
/// ============================================
/// This example demonstrates:
/// - Using describe_col() to get column metadata
/// - Extracting column information (name, type, size, nullable)
/// - Handling different SQL data types
/// - Preparing statements and inspecting result sets
///
/// Requirements:
/// - Connected to a DB2 database
/// - Table should exist: CREATE TABLE test_metadata (id INT, name VARCHAR(50), salary DECIMAL(10,2))

use ibm_db::{
    create_environment_v3,
    Statement,
    ResultSetState::{Data, NoData},
};
use std::error::Error;

fn main() {
    match test_column_metadata() {
        Ok(()) => println!("\n✓ Column metadata test completed successfully"),
        Err(e) => println!("\n✗ Column metadata test failed: {}", e),
    }
}

fn test_column_metadata() -> Result<(), Box<dyn Error>> {
    println!("\n========== COLUMN METADATA TEST ==========\n");

    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    // TODO: Update with your actual database connection details
    let connection_string = "DRIVER={IBM DB2 ODBC DRIVER};DATABASE=SAMPLE;HOSTNAME=db.example.com;PORT=50000;UID=db_user;PWD=YourPassword123";
    let conn = env.connect_with_connection_string(connection_string)?;

    println!("✓ Connected to database\n");

    // Create test table
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("CREATE TABLE IF NOT EXISTS test_metadata (id INT, name VARCHAR(50), salary DECIMAL(10,2), hire_date DATE, active INT)")? {
        Data(s) => { let _ = s.close_cursor()?; }
        NoData(_) => {}
    }
    println!("✓ Test table ready\n");

    // Test 1: Get column count and describe all columns
    println!("--- Test 1: Query Column Information ---");
    let stmt = Statement::with_parent(&conn)?;
    let prepared = stmt.prepare("SELECT * FROM test_metadata")?;

    // Get number of columns
    let col_count = prepared.num_result_cols()?;
    println!("✓ Result set has {} columns\n", col_count);

    if col_count > 0 {
        println!("Column Details:");
        println!("{:<5} {:<15} {:<15} {:<10} {:<12}", "IDX", "Name", "Type", "Size", "Nullable");
        println!("{}", "-".repeat(65));

        for i in 1..=col_count as u16 {
            match prepared.describe_col(i) {
                Ok(desc) => {
                    let type_name = format!("{:?}", desc.data_type);
                    let size = desc.column_size.unwrap_or(0);
                    let nullable = desc.nullable.unwrap_or(false);
                    let nullable_str = if nullable { "Yes" } else { "No" };

                    println!("{:<5} {:<15} {:<15} {:<10} {:<12}",
                        i,
                        desc.name,
                        type_name.chars().take(14).collect::<String>(),
                        size,
                        nullable_str
                    );
                }
                Err(e) => println!("✗ Error describing column {}: {}", i, e),
            }
        }
    }

    // Test 2: Describe column for specific query
    println!("\n--- Test 2: Specific Column Query ---");
    let stmt = Statement::with_parent(&conn)?;
    let prepared = stmt.prepare("SELECT id, name, salary FROM test_metadata WHERE id > ?")?;

    println!("Query: SELECT id, name, salary FROM test_metadata WHERE id > ?\n");
    println!("Column Info:");

    for i in 1..=prepared.num_result_cols()? as u16 {
        match prepared.describe_col(i) {
            Ok(desc) => {
                println!("Column {}: {}", i, desc.name);
                println!("  └─ Type: {:?}", desc.data_type);
                println!("  └─ Size: {:?}", desc.column_size);
                println!("  └─ Decimals: {:?}", desc.decimal_digits);
                println!("  └─ Nullable: {:?}\n", desc.nullable);
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    // Test 3: Execute query and show metadata along with data
    println!("--- Test 3: Insert Sample Data and Display ---");

    // Insert sample data
    let stmt = Statement::with_parent(&conn)?;
    let insert_sql = "INSERT INTO test_metadata (id, name, salary, hire_date, active) VALUES (1, 'John Doe', 65000.00, '2020-01-15', 1)";
    match stmt.exec_direct(insert_sql)? {
        Data(s) => { let _ = s.close_cursor()?; }
        NoData(_) => {}
    }
    println!("✓ Sample data inserted\n");

    // Query and display
    let stmt = Statement::with_parent(&conn)?;
    match stmt.exec_direct("SELECT * FROM test_metadata WHERE id = 1")? {
        Data(mut stmt) => {
            let col_count = stmt.num_result_cols()?;

            // Print header
            print!("┌");
            for i in 1..=col_count {
                let col_desc = stmt.describe_col(i as u16)?;
                print!(" {:<20}", col_desc.name);
            }
            println!(" │");

            // Print separator
            print!("├");
            for _ in 1..=col_count {
                print!(" {:<20}", "-".repeat(20));
            }
            println!(" │");

            // Fetch and print data
            let mut row_count = 0;
            while let Some(mut cursor) = stmt.fetch()? {
                print!("│");
                for i in 1..=col_count {
                    if let Ok(Some(val)) = cursor.get_data::<String>(i as u16) {
                        print!(" {:<20}", val);
                    } else {
                        print!(" {:<20}", "NULL");
                    }
                }
                println!(" │");
                row_count += 1;
            }

            // Print footer
            print!("└");
            for _ in 1..=col_count {
                print!(" {:<20}", "-".repeat(20));
            }
            println!(" ┘");

            println!("\n✓ Retrieved {} row(s)", row_count);
            let _ = stmt.close_cursor()?;
        }
        NoData(_) => println!("✗ Query returned no data"),
    }

    println!("\n========== TEST COMPLETE ==========\n");
    Ok(())
}
