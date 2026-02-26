/// Test Case: Read-Only Connection Check
/// =======================================
/// This example demonstrates:
/// - Checking if a connection is read-only
/// - Querying database properties
/// - Handling connection characteristics
///
/// Requirements:
/// - Connected to a DB2 database

use ibm_db::create_environment_v3;
use std::error::Error;

fn main() {
    match test_read_only() {
        Ok(()) => println!("✓ Read-only test completed successfully"),
        Err(e) => println!("✗ Read-only test failed: {}", e),
    }
}

fn test_read_only() -> Result<(), Box<dyn Error>> {
    println!("\n========== READ-ONLY CONNECTION TEST ==========\n");

    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    // TODO: Update with your actual database connection details
    let connection_string = "DRIVER={IBM DB2 ODBC DRIVER};DATABASE=SAMPLE;HOSTNAME=db.example.com;PORT=50000;UID=db_user;PWD=YourPassword123";
    let mut conn = env.connect_with_connection_string(connection_string)?;

    println!("✓ Connected to database\n");

    // Check if connection is read-only
    match conn.is_read_only() {
        Ok(is_readonly) => {
            if is_readonly {
                println!("ℹ Connection Mode: READ-ONLY");
                println!("  - Data source is configured as read-only");
                println!("  - Write operations will not be allowed");
            } else {
                println!("ℹ Connection Mode: READ-WRITE");
                println!("  - Data source supports both read and write operations");
                println!("  - Full CRUD operations are available");
            }
        }
        Err(e) => {
            println!("✗ Failed to determine read-only status: {}", e);
        }
    }

    println!("\n========== TEST COMPLETE ==========\n");
    Ok(())
}
