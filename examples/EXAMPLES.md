# IBM DB2 Rust Driver - Comprehensive Test Suite

This directory contains comprehensive test cases for the `ibm_db` Rust driver, demonstrating all major functionalities with well-formatted examples and detailed explanations.

## Overview

The test suite is organized by functionality, with each example focusing on a specific aspect of the DB2 driver. All examples follow a consistent, well-formatted structure for easy understanding and adaptation.

---

## Test Cases

### 1. **Basic Connection** (`connect.rs`)
**Purpose:** Demonstrate basic database connectivity and simple SELECT queries

**What it covers:**
- Creating ODBC environment
- Connecting with direct connection string
- Executing SELECT queries
- Fetching and displaying results
- User input for SQL queries

**Output Format:** Interactive, accepts SQL queries from user

---

### 2. **Advanced Query Patterns** (`advanced_queries.rs`)
**Purpose:** Demonstrate complex SQL queries and advanced query patterns

**What it covers:**
- Complex JOIN operations (INNER, LEFT, RIGHT joins)
- Subqueries and nested SELECT statements
- Filtering and sorting with WHERE and ORDER BY clauses
- String functions (UPPER, LOWER, SUBSTRING, etc.)
- Aggregate functions (COUNT, AVG, MAX, MIN, SUM)
- GROUP BY and HAVING clauses
- Statement reuse across multiple queries
- Test data setup for queries

**Key Features:**
```rust
// Complex filtering and sorting
stmt.exec_direct(
    "SELECT id, name, salary FROM employees 
     WHERE salary > 60000 
     ORDER BY salary DESC"
)?

// Aggregate functions
stmt.exec_direct(
    "SELECT department, COUNT(*) as emp_count, AVG(salary) as avg_salary 
     FROM employees 
     GROUP BY department"
)?

// JOINs
stmt.exec_direct(
    "SELECT e.name, d.dept_name 
     FROM employees e 
     INNER JOIN departments d ON e.dept_id = d.id"
)?
```

**Use Cases:**
- Data analysis and reporting
- Complex business logic queries
- Multi-table data retrieval
- Aggregation and grouping operations

---

### 3. **Transaction Control** (`transaction_control.rs`)
**Purpose:** Demonstrate transaction management with commit and rollback operations

**What it covers:**
- Disabling autocommit mode
- Creating test tables
- Executing INSERT operations
- **Committing transactions** ✓ (NEW)
- **Rolling back transactions** ✓ (NEW)
- Verifying transaction effects
- Re-enabling autocommit mode

**Key Features:**
```rust
// Disable autocommit
let mut conn = conn.disable_autocommit()?;

// Perform operations
stmt.exec_direct("INSERT INTO table ...")?;

// Commit changes
conn.commit()?;

// Or rollback
conn.rollback()?;

// Re-enable autocommit
let conn = conn.enable_autocommit()?;
```

**Output:**
```
========== TRANSACTION CONTROL TEST ==========

✓ Autocommit disabled - Transaction mode active

--- Test 1: Successful Commit ---
✓ Transaction committed successfully

--- Test 2: Rollback Transaction ---
✓ Transaction rolled back successfully
```

---

### 4. **Connection Properties** (`connection_properties.rs`)
**Purpose:** Query and display connection characteristics

**What it covers:**
- Checking if connection is read-only ✓ (NEW)
- Understanding connection mode
- Validating connection capabilities
- Connection metadata

**Key Features:**
```rust
// Check read-only status
match conn.is_read_only() {
    Ok(true) => println!("Connection is READ-ONLY"),
    Ok(false) => println!("Connection is READ-WRITE"),
    Err(e) => println!("Error: {}", e),
}
```

---

### 5. **Parameter Binding** (`parameter_binding.rs`)
**Purpose:** Demonstrate parameterized queries with bound parameters

**What it covers:**
- Preparing parameterized SQL statements
- **Binding multiple parameter types** ✓ (NEW)
  - Integers (`i32`)
  - Strings (`&str`)
  - Floating-point numbers (`f64`)
- Executing prepared statements
- **Resetting parameters for reuse** ✓ (NEW)
- Multiple executions with different values
- Parameterized SELECT/UPDATE/DELETE

**Key Features:**
```rust
// Prepare statement with placeholders
let prepared = stmt.prepare("INSERT INTO table (id, name, salary) VALUES (?, ?, ?)")?;

// Bind parameters in order
let prepared = prepared.bind_parameter(1, &id)?;
let prepared = prepared.bind_parameter(2, &name)?;
let prepared = prepared.bind_parameter(3, &salary)?;

// Execute
prepared.execute()?;

// Reset and reuse with new values
let prepared = prepared.reset_parameters()?;
```

**Security Benefits:**
- Prevents SQL injection
- Automatic type conversion
- Better performance for bulk operations

---

### 6. **Column Metadata** (`column_metadata.rs`)
**Purpose:** Retrieve and display detailed column information

**What it covers:**
- **Describing columns** ✓ (NEW)
- Getting column count
- Retrieving column properties:
  - Column names
  - Data types
  - Column sizes
  - Decimal digits
  - Nullable status
- Displaying metadata in formatted tables
- Combining metadata with query results

**Key Features:**
```rust
// Get number of columns
let col_count = stmt.num_result_cols()?;

// Describe individual columns
for i in 1..=col_count as u16 {
    let desc = stmt.describe_col(i)?;
    println!("Name: {}", desc.name);
    println!("Type: {:?}", desc.data_type);
    println!("Size: {:?}", desc.column_size);
    println!("Nullable: {:?}", desc.nullable);
}
```

**Output:**
```
Column Details:
IDX   Name            Type            Size       Nullable
---   ----            ----            ----       --------
1     id              SQL_INTEGER     0          No
2     name            SQL_VARCHAR     50         Yes
3     salary          SQL_NUMERIC     10         Yes
```

---

### 7. **Affected Row Count** (`affected_rows.rs`)
**Purpose:** Track and verify row counts from DML operations

**What it covers:**
- Getting affected row count after INSERT ✓ (NEW)
- Tracking UPDATE impact ✓ (NEW)
- Monitoring DELETE operations ✓ (NEW)
- Validating bulk operations
- Verifying operation success

**Key Features:**
```rust
// Execute and get affected rows
match stmt.exec_direct("DELETE FROM table WHERE id = 5")? {
    Data(s) => {
        let rows = s.affected_row_count()?;
        println!("Deleted {} rows", rows);
    }
    NoData(s) => {
        let rows = s.affected_row_count()?;
        println!("Operation affected {} rows", rows);
    }
}
```

**Use Cases:**
- Confirming INSERT/UPDATE/DELETE success
- Detecting silent failures
- Bulk operation monitoring
- Validation logic

---

### 8. **Data Type Handling** (`data_types.rs`)
**Purpose:** Demonstrate working with various SQL data types

**What it covers:**
- Inserting multiple data types:
  - VARCHAR (strings)
  - INT (integers)
  - DECIMAL (floating-point)
  - DATE
- Retrieving and converting data types
- **NULL value handling** ✓ (NEW)
- Type casting in SQL queries
- Aggregate functions (COUNT, AVG, MAX, MIN)
- Safe type conversions

**Key Features:**
```rust
// Retrieve different types
let id: Option<i32> = cursor.get_data(1)?;
let name: Option<String> = cursor.get_data(2)?;
let salary: Option<f64> = cursor.get_data(3)?;

// Handle NULL safely
if let Some(value) = id {
    println!("ID: {}", value);
} else {
    println!("ID is NULL");
}
```

**Output:**
```
ID    Name             Salary       Active
---   ----             ------       ------
1     John Doe         $85000.00    1
2     Jane Smith       $78500.50    1
NULL  Unknown Emp      NULL         NULL
```

---

### 9. **Error Handling** (`error_handling.rs`)
**Purpose:** Demonstrate proper error handling and recovery

**What it covers:**
- Catching SQL syntax errors
- Handling non-existent tables
- Type mismatch errors
- NULL value handling
- Division by zero
- Parameter binding validation
- Error recovery patterns
- Graceful degradation

**Key Features:**
```rust
// Handle connection errors
match env.connect_with_connection_string(conn_str) {
    Ok(conn) => { /* use connection */ }
    Err(e) => println!("Connection failed: {}", e),
}

// Handle query errors
match stmt.exec_direct(sql) {
    Ok(Data(mut s)) => { /* process results */ }
    Ok(NoData(s)) => { /* handle no results */ }
    Err(e) => println!("Query error: {}", e),
}

// Handle parameter binding errors
match prepared.bind_parameter(1, &value) {
    Ok(p) => { /* continue */ }
    Err(e) => println!("Binding error: {}", e),
}
```

**Error Recovery:**
```rust
let operations = vec![
    "SELECT * FROM valid_table",
    "SELECT * FROM invalid_table",
    "SELECT * FROM valid_table",  // Can still execute this
];

for sql in operations {
    match stmt.exec_direct(sql) {
        Ok(_) => println!("Success"),
        Err(_) => println!("Error (recovered)"),  // Continue with next
    }
}
```

---

### 10. **List Tables** (`list_tables.rs`)
**Purpose:** Query database schema and list available tables

**What it covers:**
- Using `tables()` or `tables_str()` method
- Filtering by catalog, schema, table name, table type
- Iterating through result sets
- Pretty-printing table information

---

### 11. **Column Descriptions** (`column_descriptions.rs`)
**Purpose:** Get detailed information about columns in a result set

**What it covers:**
- Executing queries
- Using `describe_col()` on result sets
- Accessing column metadata
- Column type information

---

### 12. **Bind Parameters** (`bind_params.rs`)
**Purpose:** Demonstrate parameter binding in prepared statements

**What it covers:**
- Preparing statements with `?` placeholders
- Binding parameters
- Executing with bound values
- Multiple executions with different parameters

---

### 13. **Connection Pool** (`connect_pool.rs`)
**Purpose:** Demonstrate connection pooling using r2d2

**What it covers:**
- Creating connection pools
- Managing multiple connections
- Concurrent access patterns
- Connection reuse

---

### 14. **Execute Query** (`execute_query.rs`)
**Purpose:** Execute queries using connection pooling

**What it covers:**
- Executing queries from a pool
- Handling pooled connections
- Query execution patterns

---

### 15. **Print Drivers and Datasources** (`print_drivers_and_datasources.rs`)
**Purpose:** List available ODBC drivers and data sources

**What it covers:**
- Enumerating drivers
- Listing data sources
- System vs. user data sources

---

## Note on Setup

The **Setup utility** (`setup.rs`) is located in `src/bin/setup.rs`, NOT in the examples folder.

**Purpose:** Initialize the environment and download CLI drivers

**What it covers:**
- One-time setup for downloading IBM DB2 CLI driver
- Cross-platform driver installation (Windows, Linux, macOS)
- Automatic architecture detection (32-bit/64-bit)
- Environment configuration

**How to run:**
```bash
cargo run --bin setup
```

See the main [README.md](../README.md#installation) for complete Setup instructions.

---

## Running the Examples

### Prerequisites
1. Rust installed (1.45+)
2. IBM DB2 CLI Driver installed
3. Database connectivity configured
4. Update connection string with actual database details

### Run a specific example
```bash
# Basic connection
cargo run --example connect

# Transaction control
cargo run --example transaction_control

# Parameter binding
cargo run --example parameter_binding

# Data types
cargo run --example data_types

# Error handling
cargo run --example error_handling

# Column metadata
cargo run --example column_metadata

# Affected rows
cargo run --example affected_rows

# Connection properties
cargo run --example connection_properties

# List tables
cargo run --example list_tables
```

### Run all examples
```bash
cargo test --examples
```

---

## Connection String Format

```rust
let connection_string = 
    "DRIVER={IBM DB2 ODBC DRIVER};\
     DATABASE=SAMPLE;\
     HOSTNAME=your-host.com;\
     PORT=60000;\
     UID=username;\
     PWD=password";
```

**Parameters:**
- `DRIVER`: ODBC driver name
- `DATABASE`: Database name
- `HOSTNAME`: Server hostname or IP
- `PORT`: ODBC port (default: 50000 for DB2)
- `UID`: Username
- `PWD`: Password

---

## Key Concepts Demonstrated

### 1. **Autocommit vs Transaction Mode**
```rust
// Autocommit mode (default)
let conn = env.connect_with_connection_string(conn_str)?;
stmt.exec_direct("INSERT ...")?;  // Auto-committed

// Transaction mode
let mut conn = conn.disable_autocommit()?;
stmt.exec_direct("INSERT ...")?;
conn.commit()?;  // Manual commit
```

### 2. **Result Set Handling**
```rust
match stmt.exec_direct(sql)? {
    Data(mut stmt) => {
        while let Some(mut cursor) = stmt.fetch()? {
            let value = cursor.get_data::<String>(1)?;
        }
    }
    NoData(_) => println!("No data returned"),
}
```

### 3. **Prepared Statements**
```rust
let prepared = stmt.prepare("SELECT * FROM table WHERE id = ?")?;
let prepared = prepared.bind_parameter(1, &id)?;
prepared.execute()?;
```

### 4. **Error Handling**
```rust
match operation() {
    Ok(result) => { /* success */ }
    Err(e) => { /* error */ }
}
```

---

## Best Practices

1. **Always handle errors** - Use `Result` and `?` operator
2. **Close result sets** - Call `close_cursor()` before statement reuse
3. **Use prepared statements** - Better performance and security for parameterized queries
4. **Handle NULL values** - Check `Option<T>` results
5. **Use transactions** - For multi-statement operations requiring atomicity
6. **Validate input** - Especially for dynamic SQL constructions
7. **Pool connections** - For multi-threaded applications
8. **Close connections** - Call `disconnect()` or let it drop implicitly

---

## Testing Checklist

- [ ] Test connection establishment
- [ ] Test transaction commit
- [ ] Test transaction rollback
- [ ] Test parameter binding with various types
- [ ] Test column metadata retrieval
- [ ] Test affected row counts
- [ ] Test NULL value handling
- [ ] Test error handling and recovery
- [ ] Test connection properties
- [ ] Test table enumeration

---

## Additional Resources

- [IBM DB2 Documentation](https://www.ibm.com/docs/db2)
- [ODBC Reference](https://docs.microsoft.com/en-us/sql/odbc/reference/odbc-reference)
- [Rust IBM_DB Crate](https://docs.rs/ibm_db/)
- [R2D2 Connection Pool](https://docs.rs/r2d2/)

---

## Support

For issues or questions:
1. Check the error output - Most errors include diagnostic information
2. Review relevant example code
3. Verify database connectivity
4. Check credentials and permissions
5. Consult DB2 documentation for specific database errors
