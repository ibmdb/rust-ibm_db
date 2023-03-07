# rust_ibm_db

Interface for Rust to DB2 for z/OS, DB2 for LUW, DB2 for i with support for Connection Pooling using r2d2.

## API Documentation

> For complete list of rust_ibm_db APIs refer to this link: https://docs.rs/ibm_db/1.0.5/ibm_db/

## Testing IBM_DB driver on Linux

### Prerequisites

We tested the following steps on an AWS EC2 instance rrunning AL2.

#### Install dependencies
```
# yum -y install gcc unixODBC git openssl-devel
```

#### Install Rust

```
$ curl https://sh.rustup.rs -sSf | sh
$ source ~/.bashrc

$ cargo --version
$ rustc --version
$ git --version
```

#### Install **clidriver**.

<pre>
<b>$ cargo install ibm_db --example setup</B>

... more ...
    Finished release [optimized] target(s) in 1m 40s
  Installing /home/admin/.cargo/bin/setup
   Installed package `ibm_db v1.0.5` (executable `setup`)

<b>$ /home/admin/.cargo/bin/setup</b> 
</pre>

#### Add entries to profile

<pre><b>
$ echo "export IBM_DB_HOME=$HOME/clidriver" >> ~/.bashrc
$ echo 'export LD_LIBRARY_PATH="${IBM_DB_HOME}/lib"' >> ~/.bashrc
$ echo 'export PATH="${IBM_DB_HOME}/bin":$PATH' >> ~/.bashrc
$ source ~/.bashrc
</B></pre>

#### Install **ibm_db**.

<pre><b>
$ cargo install ibm_db
</b></pre>

#### Create a Rust package

<pre><b>
$ cargo new testdb
</b></pre>

#### Add ibm_db as a dependency to Cargo.toml

<pre><b>
$ cd testdb
$ echo 'ibm_db = "1.0.5"' >> Cargo.toml
</b></pre>

#### Create connection properties using db2cli command

> Note: Change connection properties as per your database endpoints

<pre><b>
$ cd ~/clidriver/bin
$ ./db2cli writecfg add \
-dsn MYDB \
-database TESTDB \
-host database-1.xxxxxxx.us-east-1.rds.amazonaws.com \
-port 50000
</b></pre>

#### Check db2dsdriver.cfg file

<pre><b>
cat ~/clidriver/cfg/db2dsdriver.cfg
</b></pre>

#### Replace ~/testdb/src/main.rs with the following program

```
use ibm_db::{safe::AutocommitOn,Statement,create_environment_v3, Connection,ResultSetState::{NoData, Data}};
use std::error::Error;

fn main() {
    match connect() {
        Ok(()) => println!("Success."),
        Err(diag) => println!("Error: {}", diag),
    }
}

fn connect() -> Result<(), Box<dyn Error>> {

    let env = create_environment_v3().map_err(|e| e.unwrap())?;
    let conn = env.connect("MYDB", "admin", "password").unwrap();
    println!("Connection successful.");
    execute_statement(&conn)
}

fn execute_statement<'env>(conn: &Connection<'env, AutocommitOn>) -> Result<(),Box<dyn Error>> {
    let stmt = Statement::with_parent(conn)?;

    let sql_text = "select current server from sysibm.dual";

    match stmt.exec_direct(&sql_text)? {
        Data(mut stmt) => {
            let cols = stmt.num_result_cols()?;
            while let Some(mut cursor) = stmt.fetch()? {
                for i in 1..(cols + 1) {
                    match cursor.get_data::<&str>(i as u16)? {
                        Some(val) => print!(" {}", val),
                        None => print!(" NULL"),
                    }
                }
                println!();
            }
        }
        NoData(_) => println!("Query executed, no data returned"),
    }

    Ok(())
}
```

#### Build and run the program

<pre>
<b>$ cd ~/testdb
$ cargo build</b>
... more ...
   Compiling testdb v0.1.0 (/home/admin/testdb)
    Finished dev [unoptimized + debuginfo] target(s) in 0.44s

<b>$ ./target/debug/testdb</b>

Connection successful.
 TESTDB
Success.
</pre>

You successfully tested the Rust program for connecting to the RDS Db2.

### <a name="Licenserequirements"></a> License requirements for connecting to databases

rust_ibm_db driver can connect to DB2 on Linux Unix and Windows without any additional license/s, however, connecting to databases on DB2 for z/OS or DB2 for i(AS400) Servers require either client side or server side license/s. The client side license would need to be copied under `license` folder of your `clidriver` installation directory and for activating server side license, you would need to purchase DB2 Connect Unlimited for System z® and DB2 Connect Unlimited Edition for System i®.

To know more about license and purchasing cost, please contact [IBM Customer Support](http://www-05.ibm.com/support/operations/zz/en/selectcountrylang.html).

To know more about server based licensing viz db2connectactivate, follow below links:
* [Activating the license certificate file for DB2 Connect Unlimited Edition](https://www.ibm.com/developerworks/community/blogs/96960515-2ea1-4391-8170-b0515d08e4da/entry/unlimited_licensing_in_non_java_drivers_using_db2connectactivate_utlility1?lang=en).
* [Unlimited licensing using db2connectactivate utility](https://www.ibm.com/developerworks/community/blogs/96960515-2ea1-4391-8170-b0515d08e4da/entry/unlimited_licensing_in_non_java_drivers_using_db2connectactivate_utlility1?lang=en.)


## Contributing to the <a name='contributing-to-the-ibm_db-RUST-project'>ibm_db</a> RUST project

See [CONTRIBUTING](https://github.com/ibmdb/rust-ibm_db/blob/main/CONTRIBUTING.md)

```
The developer sign-off should include the reference to the DCO in remarks(example below):
DCO 1.1 Signed-off-by: Random J Developer <random@developer.org>
```