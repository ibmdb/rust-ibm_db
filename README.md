# rust_ibm_db

[![Crates.io](https://img.shields.io/crates/v/ibm_db.svg)](https://crates.io/crates/ibm_db)
[![Docs.rs](https://docs.rs/ibm_db/badge.svg)](https://docs.rs/ibm_db/)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.45%2B-orange.svg)](https://www.rust-lang.org/)
[![GitHub](https://img.shields.io/badge/github-rust--ibm_db-blue.svg)](https://github.com/ibmdb/rust-ibm_db)

> A Rust interface for connecting to **IBM DB2** databases (z/OS, LUW, i) with connection pooling support using r2d2.

## 📋 Table of Contents

- [Quick Start](#quick-start-for-beginners)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Running Examples](#running-examples)
- [Documentation](#documentation)
- [Troubleshooting](#troubleshooting)
- [License Requirements](#license-requirements)
- [Contributing](#contributing)

## Quick Start for Beginners

If you're new to Rust and want to get started quickly:

1. **[Install Prerequisites](#prerequisites)** (15 minutes)
2. **[Run Setup Utility](#installation)** (5-10 minutes)
3. **[Configure Environment](#configuration)** (5 minutes)
4. **[Run First Example](#running-examples)** (2 minutes)

**Total time: ~30 minutes to your first database connection!**

## Prerequisites

### Required Software

Before starting, make sure you have installed:

- **Rust** (version 1.45 or newer) — [Install Rust](https://www.rust-lang.org/tools/install)
- **Git** — [Install Git](https://git-scm.com/downloads)
- **A C compiler**:
  - **Windows**: Microsoft Visual Studio Build Tools
  - **Linux**: GCC or Clang (usually pre-installed)
  - **macOS**: Xcode Command Line Tools

### Verify Your Setup

Open a terminal/command prompt and run:

```bash
rustc --version     # Should show Rust version ≥ 1.45
git --version       # Should show Git version
```

If either command fails, follow the installation links above.

### System Dependencies

Depending on your operating system, you'll need to install additional libraries. These contain the necessary files for database connections.

#### 🪟 Windows

```powershell
# Option 1: Using Chocolatey (recommended)
choco install openssl

# Option 2: Manual installation
# Download from: https://slproweb.com/products/Win32OpenSSL.html
# Choose "Win64 OpenSSL v3.0.x" (for 64-bit Windows)
```

> ✅ **ODBC is included:** Windows comes with ODBC development files pre-installed, so you don't need to install anything extra!

#### 🐧 Linux (Ubuntu/Debian)

```bash
# Install OpenSSL and ODBC development files
sudo apt-get update
sudo apt-get install -y libssl-dev unixodbc-dev
```

#### 🐧 Linux (RedHat/CentOS/Fedora)

```bash
# Install OpenSSL and ODBC development files
sudo yum install -y openssl-devel unixODBC-devel
```

#### 🍎 macOS

```bash
# Install Homebrew (if not already installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install OpenSSL and ODBC
brew install openssl unixodbc

# Set up environment variables (add to your shell profile)
echo 'export LDFLAGS="-L/usr/local/opt/openssl/lib"' >> ~/.zshrc
echo 'export CPPFLAGS="-I/usr/local/opt/openssl/include"' >> ~/.zshrc
source ~/.zshrc
```

### Verify System Dependencies

Check that your system dependencies are installed:

**Windows (Command Prompt):**
```batch
openssl version
```

**Linux/macOS (Terminal):**
```bash
openssl version
```

You should see output like: `OpenSSL 3.0.x ...`

---

## Installation

### Step 1: Get the Code

```bash
# Clone the repository (or download the zip file)
git clone https://github.com/ibmdb/rust-ibm_db.git
cd rust-ibm_db
```

### Step 2: Download & Install IBM DB2 CLI Driver

The setup utility will automatically download the correct driver for your system. Simply run:

```bash
cargo run --bin setup
```

**What happens:**
- ✅ Detects your operating system (Windows / Linux / macOS)
- ✅ Detects your system architecture (32-bit / 64-bit)
- ✅ Downloads the appropriate IBM DB2 CLI Driver
- ✅ Extracts the files to your system
- ✅ Sets up environment variables (see next step)

**Expected output:**
```
Setting up IBM DB2 CLI Driver...
Downloading CLI Driver for your platform...
[Download progress...]
Installation complete! IBM_DB_HOME is now set to: /path/to/clidriver
```

### Step 3: Verify Installation

Let's make sure everything was installed correctly:

**Windows (Command Prompt):**
```batch
echo %IBM_DB_HOME%    # Should print the path
dir %IBM_DB_HOME%     # Should show many files
```

**Linux/macOS (Terminal):**
```bash
echo $IBM_DB_HOME       # Should print the path
ls -la $IBM_DB_HOME     # Should show many files
```

Both commands should show output. If you see errors, check [Troubleshooting](#troubleshooting).

### Step 4: Install the ibm_db Crate

Now that the CLI driver is installed, you need to install the Rust crate that provides the Rust API.

**Option A: Install from crates.io (recommended)**
```bash
cargo install ibm_db
```

**Option B: Install from current repository**
```bash
cargo install --path .
```

**Why two options?**
- Option A: Always gets the latest published version from crates.io
- Option B: Installs from the repo you cloned, useful if you're modifying the code

✅ **Installation complete!** You now have:
- ✅ IBM DB2 CLI Driver installed
- ✅ ibm_db Rust crate installed
- ✅ Environment variables configured

---

### When is setup.rs Needed?

**You ONLY need setup.rs if:**
- ✅ The CLI driver is NOT yet installed
- ✅ You're setting up on a fresh machine

**You DON'T need setup.rs if:**
- ✅ CLI driver is already installed
- ✅ You're using a project that already has the driver
- ✅ You're installing from crates.io

---

## Configuration

### Setting Environment Variables

After the setup utility completes, your environment variables should be configured. However, you may need to set them manually in a new terminal session. Follow the steps for your operating system:

#### 🪟 Windows (Command Prompt)

**Temporary (current session only):**
```batch
set IBM_DB_HOME=C:\IBM\IBM_DATA_SERVER_DRIVER\clidriver
set PATH=%IBM_DB_HOME%\bin;%PATH%
```

**Permanent (recommended):**
1. Press `Win + RBreak` to open System Properties
2. Click **Environment Variables**
3. Click **New** (under System variables)
4. Add two variables:
   - Name: `IBM_DB_HOME` | Value: `C:\IBM\IBM_DATA_SERVER_DRIVER\clidriver`
   - Name: `PATH` | Value: `C:\IBM\IBM_DATA_SERVER_DRIVER\clidriver\bin;` (prefix to existing PATH)
5. Click **OK** and restart your terminal

#### 🐧 Linux (Bash/Zsh)

**Temporary (current session):**
```bash
export IBM_DB_HOME=/IBM/IBM_DATA_SERVER_DRIVER/clidriver
export LD_LIBRARY_PATH=$IBM_DB_HOME/bin:$LD_LIBRARY_PATH
```

**Permanent (recommended):**
```bash
# Add to ~/.bashrc or ~/.zshrc
echo 'export IBM_DB_HOME=/IBM/IBM_DATA_SERVER_DRIVER/clidriver' >> ~/.bashrc
echo 'export LD_LIBRARY_PATH=$IBM_DB_HOME/bin:$LD_LIBRARY_PATH' >> ~/.bashrc
source ~/.bashrc
```

#### 🍎 macOS (Zsh/Bash)

**Temporary (current session):**
```bash
export IBM_DB_HOME=/IBM/IBM_DATA_SERVER_DRIVER/clidriver
export DYLD_LIBRARY_PATH=$IBM_DB_HOME/bin:$DYLD_LIBRARY_PATH
```

**Permanent (recommended):**
```bash
# For Zsh (default in modern macOS)
echo 'export IBM_DB_HOME=/IBM/IBM_DATA_SERVER_DRIVER/clidriver' >> ~/.zshrc
echo 'export DYLD_LIBRARY_PATH=$IBM_DB_HOME/bin:$DYLD_LIBRARY_PATH' >> ~/.zshrc
source ~/.zshrc

# OR for Bash
echo 'export IBM_DB_HOME=/IBM/IBM_DATA_SERVER_DRIVER/clidriver' >> ~/.bash_profile
echo 'export DYLD_LIBRARY_PATH=$IBM_DB_HOME/bin:$DYLD_LIBRARY_PATH' >> ~/.bash_profile
source ~/.bash_profile
```

### Connecting to Your Database

When you connect to a database, you'll need a **connection string**. This tells the driver where to find your database.

**Connection String Format:**
```
DRIVER={IBM DB2 ODBC DRIVER};DATABASE=mydb;HOSTNAME=server.com;PORT=50000;UID=myuser;PWD=mypassword
```

**Breaking it down:**
- `DRIVER={IBM DB2 ODBC DRIVER}` — Always use this (it's the driver name)
- `DATABASE=mydb` — Your database name
- `HOSTNAME=server.com` — Your server address
- `PORT=50000` — Your database port
- `UID=myuser` — Your username
- `PWD=mypassword` — Your password

**Example with real values:**
```rust
let connection_string = "DRIVER={IBM DB2 ODBC DRIVER};DATABASE=SAMPLE;HOSTNAME=db.mycompany.com;PORT=60000;UID=admin;PWD=secretpass123";
```

---

## Running Examples

### Important: Set IBM_DB_HOME Before Running

To use the CLI driver in your cargo commands, you must set the `IBM_DB_HOME` environment variable:

**Windows (Command Prompt):**
```batch
cargo run --package ibm_db --example connect
```
_IBM_DB_HOME should already be set from Configuration section_

**Linux/macOS (Terminal):**
```bash
IBM_DB_HOME=. cargo run --package ibm_db --example connect
```
_Or use the path where you installed the clidriver_

### Your First Program: Connect to Database

**Windows:**
```batch
cargo run --package ibm_db --example connect
```

**Linux/macOS:**
```bash
IBM_DB_HOME=. cargo run --package ibm_db --example connect
```

You'll be prompted to enter a SQL query. Try:
```sql
SELECT * FROM SYSCAT.TABLES
```

### Other Examples

The project includes many examples to learn from:

| Example | What it does |
|---------|-------------|
| `connect` | Basic database connection |
| `list_tables` | Lists all tables in a database |
| `execute_query` | Runs SQL queries |
| `bind_params` | Uses parameterized queries (safe for user input) |
| `transaction_control` | Demonstrates transactions (commit/rollback) |
| `data_types` | Shows how to work with different DB2 data types |
| `error_handling` | Proper error handling patterns |
| `connect_pool` | Connection pooling for performance |

**Run any example:**

**Windows:**
```batch
cargo run --package ibm_db --example <example_name>
```

**Linux/macOS:**
```bash
IBM_DB_HOME=. cargo run --package ibm_db --example <example_name>
```

**Example commands:**
```bash
# Windows
cargo run --package ibm_db --example connect
cargo run --package ibm_db --example list_tables

# Linux/macOS
IBM_DB_HOME=. cargo run --package ibm_db --example connect
IBM_DB_HOME=. cargo run --package ibm_db --example list_tables
```

---

## Documentation

- **Complete API Reference**: https://docs.rs/ibm_db/
- **Crates.io Page**: https://crates.io/crates/ibm_db
- **GitHub Repository**: https://github.com/ibmdb/rust-ibm_db
- **Examples Folder**: See `examples/` directory in this repository

---

## How to Use in Your Own Project

### Step 1: Create a New Rust Project

```bash
cargo new my_db_app
cd my_db_app
```

### Step 2: Add IBM DB2 to Cargo.toml

Edit `Cargo.toml` and add:

```toml
[dependencies]
ibm_db = "1.0.6"
```

### Step 3: Write Code to Connect

Edit `src/main.rs`:

```rust
use ibm_db::environment::Environment;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an ODBC environment
    let env = Environment::new()?;
    
    // Connect to the database
    let connection_string = "DRIVER={IBM DB2 ODBC DRIVER};DATABASE=SAMPLE;HOSTNAME=your.server.com;PORT=60000;UID=youruser;PWD=yourpass";
    let conn = env.connect_with_connection_string(connection_string)?;
    
    println!("✅ Connected to database!");
    
    Ok(())
}
```

### Step 4: Run It!

**Windows:**
```batch
REM Make sure IBM_DB_HOME is set (from Configuration section)
cargo run
```

**Linux/macOS:**
```bash
IBM_DB_HOME=. cargo run
```

You should see: `✅ Connected to database!`

---

## Components Overview

### What's Inside?

This project contains several key parts (don't worry if you don't understand all of this yet):

| Component | Purpose | Example Use |
|-----------|---------|------------|
| **Setup Utility** | Downloads and installs the DB2 driver | Run once: `cargo run --bin setup` |
| **Connection Module** | Opens/manages database connections | Used internally by functions |
| **ODBC Interface** | Communicates with the database | Used internally by functions |
| **Statement Processor** | Executes SQL queries | `execute_query.rs` example |
| **Error Handler** | Shows detailed error messages | When something goes wrong |
| **Connection Pooling** | Reuses connections efficiently | For advanced applications |

### Technology Stack (In Plain English)

- **ODBC**: The "universal language" that lets Rust talk to database drivers
- **OpenSSL**: Encryption library for secure communication
- **IBM DB2 CLI Driver**: The translator between ODBC and the actual DB2 database

## Prerequisite

> RUST should be installed (Rust version should be >=1.45)
Confirm by typing below in command prompt:

```
>rustc --version
```

> GIT should be installed
Confirm by typing below in command prompt:

```
>git --version
```

### System Dependencies

#### Windows
- **OpenSSL**: Download and install from https://slproweb.com/products/Win32OpenSSL.html (or use `choco install openssl`)
- **ODBC Development Files**: Included with Windows SDK

#### Linux (Ubuntu/Debian)
```bash
# Install OpenSSL development files
sudo apt-get install libssl-dev

# Install ODBC development files
sudo apt-get install unixodbc-dev
```

#### Linux (RedHat/CentOS/Fedora)
```bash
# Install OpenSSL development files
sudo yum install openssl-devel

# Install ODBC development files
sudo yum install unixODBC-devel
```

#### macOS
```bash
# Install OpenSSL using Homebrew
brew install openssl

# Install ODBC (if not already installed)
brew install unixodbc
```

After installing OpenSSL on macOS, you may need to set environment variables:
```bash
export LDFLAGS="-L/usr/local/opt/openssl/lib"
export CPPFLAGS="-I/usr/local/opt/openssl/include"
```

---

## OpenSSL Configuration Details

### Why Is OpenSSL Important?

OpenSSL handles the security aspects of your database connection. It ensures:
- 🔒 **Encryption**: Your data is scrambled in transit
- 🔐 **Authentication**: The server proves it's really who it claims to be
- ✅ **Security**: Prevents eavesdropping and man-in-the-middle attacks

### Verify OpenSSL is Working

**Windows:**
```batch
openssl version -a
where openssl
```

**Linux/macOS:**
```bash
openssl version -a
pkg-config --cflags --libs openssl
```

_If you see error messages, go back to the [System Dependencies](#system-dependencies) section._

### Common OpenSSL Problems & Fixes

| ❌ Problem | 🔍 Reason | ✅ Solution |
|-----------|----------|-----------|
| `undefined reference to 'SSL_CTX_new'` | OpenSSL not installed | Install OpenSSL (see System Dependencies) |
| `library not found for -lssl` (macOS) | OpenSSL in wrong location | Run: `export LDFLAGS="-L/usr/local/opt/openssl/lib"` |
| `libssl.so.1.1: cannot open shared object` | Missing runtime library | Ubuntu: `sudo apt-get install libssl1.1` |

---

## Troubleshooting

### Windows Errors

| ❌ Error | 🔧 What to Try |
|---------|--------------|
| `"libssl-1_1-x64.dll not found"` | Install OpenSSL: https://slproweb.com/products/Win32OpenSSL.html |
| `"IBM DB2 ODBC DRIVER not found"` | Set `IBM_DB_HOME` and `PATH` (see Configuration section) |
| `"The setup.rs crashed/hung"` | Close the terminal and try again. Check internet connection. |

### Linux Errors

| ❌ Error | 🔧 What to Try |
|---------|--------------|
| `"libssl.so.1.1: cannot open shared object file"` | Run: `sudo apt-get install libssl1.1` |
| `"unixodbc-dev not found"` | Run: `sudo apt-get install unixodbc-dev` |
| `"LD_LIBRARY_PATH not persisting"` | Add export statements to `~/.bashrc` and run: `source ~/.bashrc` |

### macOS Errors

| ❌ Error | 🔧 What to Try |
|---------|--------------|
| `"dyld: Library not loaded: libdb2.dylib"` | Set `DYLD_LIBRARY_PATH` before running (see Configuration section) |
| `"library not found for -lssl"` | Set: `export LDFLAGS="-L/usr/local/opt/openssl/lib"` |
| `"Homebrew OpenSSL not found"` | Run: `brew install openssl` and set LDFLAGS/CPPFLAGS |

### General Tips

**Still have issues? Try these steps:**

1. **Close and reopen your terminal** — Updates to environment variables won't appear until you do this
2. **Verify each step** — Run the verification commands in the Configuration section
3. **Check internet connection** — The setup utility needs to download the CLI Driver (100+ MB)
4. **Look for typos** — Environment variable paths are case-sensitive on Linux/macOS
5. **Ask for help** — Visit https://github.com/ibmdb/rust-ibm_db/issues

---

## License Requirements

### Free Databases

You can connect to these **without additional licenses**:
- ✅ DB2 for Linux/Unix/Windows (LUW)
- ✅ DB2 on premises
- ✅ IBM Cloud Db2

### Licensed Databases

Connecting to these **requires purchased licenses**:
- ⚠️ DB2 for z/OS (mainframe)
- ⚠️ DB2 for i (AS/400)

**What to do if you need a license:**
1. Contact [IBM Customer Support](http://www-05.ibm.com/support/operations/zz/en/selectcountrylang.html)
2. Purchase **DB2 Connect Unlimited** for your platform
3. Copy the license file to: `$IBM_DB_HOME/license/`

---

## Contributing

We welcome contributions! Whether you find a bug, want to add a feature, or improve documentation:

1. **Report Issues**: https://github.com/ibmdb/rust-ibm_db/issues
2. **Submit Pull Requests**: https://github.com/ibmdb/rust-ibm_db/pulls
3. **Read Guidelines**: See [CONTRIBUTING.md](CONTRIBUTING.md)

When contributing, please include:
- What problem you're solving
- How to test your changes
- Any new dependencies (with versions)

### Developer Sign-Off

All contributions require a sign-off referencing the DCO:
```
DCO 1.1 Signed-off-by: Your Name <your.email@example.com>
```

---

## Project Structure

```
rust-ibm_db/
├── src/
│   ├── lib.rs                 # Main library file
│   ├── connection.rs          # Database connection logic
│   ├── statement/             # SQL statement execution
│   ├── environment/           # ODBC environment setup
│   ├── diagnostics.rs         # Error handling
│   └── ffi.rs                 # Low-level C bindings
├── src/bin/
│   └── setup.rs               # CLI Driver download utility
├── examples/
│   ├── connect.rs             # Basic connection example
│   ├── execute_query.rs       # Run SQL example
│   ├── bind_params.rs         # Parameterized queries
│   └── ...                    # More examples
├── build.rs                   # Build script (compiles C code)
├── Cargo.toml                 # Project dependencies
└── README.md                  # This file
```

---

## Resources for Learning

### Rust Fundamentals
- [The Rust Book](https://doc.rust-lang.org/book/) - Official Rust guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by doing
- [Rustlings](https://github.com/rust-lang/rustlings) - Micro-lessons

### DB2 & Databases
- [IBM DB2 Documentation](https://www.ibm.com/products/db2)
- [ODBC Documentation](https://en.wikipedia.org/wiki/Open_Database_Connectivity)
- [SQL Tutorial](https://www.w3schools.com/sql/) - Learn SQL basics

### rust_ibm_db Specific
- [API Documentation](https://docs.rs/ibm_db/) - Full function reference
- [GitHub Repository](https://github.com/ibmdb/rust-ibm_db) - Source code
- [Examples Folder](examples/) - Real code examples

---

## Frequently Asked Questions (FAQ)

### Q: Do I need to know Rust to use this?
**A:** You need basic Rust knowledge. Check out [The Rust Book](https://doc.rust-lang.org/book/) for a quick introduction.

### Q: What if I can't install the CLI Driver?
**A:** The setup utility automates this, but if it fails: Check your internet, ensure you have 500MB free disk space, and see Troubleshooting section.

### Q: Can I use this on macOS?
**A:** Yes! Ensure you install OpenSSL via Homebrew and set the LDFLAGS/CPPFLAGS environment variables.

### Q: Do I need to set environment variables in every terminal?
**A:** No, if you make the changes permanent (see Configuration section), they'll persist.

### Q: What if my database uses a non-standard port?
**A:** That's fine! Just include the port in your connection string (e.g., `PORT=52000`).

### Q: How do I secure my database password?
**A:** Never hardcode passwords! Use environment variables:
```rust
let password = std::env::var("DB_PASSWORD")?;
let conn_str = format!("DRIVER={{IBM DB2 ODBC DRIVER}};...;PWD={}", password);
```

---

## Related Projects

- [ibm_db (Python)](https://github.com/ibmdb/python-ibmdb) - IBM DB2 for Python
- [ibm_db_nodejs](https://github.com/ibmdb/node-ibm_db) - IBM DB2 for Node.js
- [ibm_db_go](https://github.com/ibmdb/go-ibm_db) - IBM DB2 for Go

---

## License

This project is licensed under the **Apache License 2.0**. See [LICENSE](LICENSE) file for details