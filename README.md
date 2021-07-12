# rust_ibm_db

Interface for Rust to DB2 for z/OS, DB2 for LUW, DB2 for i with support for Connection Pooling using r2d2.

## API Documentation

> For complete list of rust_ibm_db APIs refer to ODBC documentation

## Prerequisite

> RUST should be installed(Rust version should be >=1.45)
Confirm by typing below in command prompt:

```
>rustc --version

```
> GIT should be installed
Confirm by typing below in command prompt:

```
>git --version
```

## How to Install

### Method-1(Using GIT Repo):
> We'll assume that you've installed Git, forked [rust-ibm_db](https://github.com/ibmdb/rust-ibm_db.git), and cloned the forked repo to your PC.
>
> If not, using the command line interface to interact with Git,
> you can get the rust driver to your machine by running the below git command i.e.
```
git clone https://github.com/ibmdb/rust-ibm_db
```
>
> There are also a number of GUIs and IDE integrations that can generally do the same things.
>
> If you've cloned your fork, then you will be able to reference it with origin in your local repo.

> CLI Driver should be downloaded in your system and IBM_DB_PATH, LD_LIBRARY_PATH should be set to point to CLI Driver folder.
>
### Method-2(Downloading ibm_db crate from crates.io):
> Download the ibm_db crate from crates.io using the below link:
```
https://crates.io/api/v1/crates/ibm_db/<version to be downloaded i.e. 1.0.1 or 1.0.2 etc.>/download
```
> Once done, unzip the .crate file which is actually a .tar.gz.
>
> Copy the setup.rs from "examples" folder to your RUST project under examples folder.
> 
> Add the below dependencies in case not present:
```
[build-dependencies]
cc = "1.0"
winapi = "0.2"
user32-sys = "0.2"
sys-info = "0.7.0"
bitness = "0.4.0"
error-chain = "0.12.4"
tempfile = "3.1.0"
reqwest = "0.10.10"
tokio = { version = "0.2", features = ["full"] }
futures = "0.3.8"
zip = "0.5"
flate2 = "1.0"
tar = "0.4"

[dependencies]
winapi = "0.2"
user32-sys = "0.2"
sys-info = "0.7.0"
bitness = "0.4.0"
error-chain = "0.12.4"
tempfile = "3.1.0"
reqwest = "0.10.10"
tokio = { version = "0.2", features = ["full"] }
futures = "0.3.8"
zip = "0.5"
flate2 = "1.0"
tar = "0.4"
odbc-safe = "0.5.0"
odbc-sys = "0.8.2"
log = "0.4.1"
encoding_rs = "0.8.14"
prettytable-rs = "^0.8"
lazy_static = "1.0"
r2d2 = "0.8"
```
> 
> If CLI Driver is not installed run the below command once you checkout the GIT repo and it will be installed:
```
cargo run --package <package name i.e. ibm_db or <your package name>> --example setup
```

> Then you can do "cargo install --path ." from ibm_db crate or "cargo install ibm_db" or simply include the "ibm_db" driver in your cargo.toml depending on your convenience. 

#### NOTE: 

In order for the test/db program to run, DSN needs to be configured. 
Update the db2dsdriver.cfg file(present in /clidriver/cfg folder under CLI driver path) with the requisite details.

Example as follows:
```
e.g.
<?xml version="1.0" encoding="UTF-8" standalone="no" ?>
<configuration>
  <dsncollection>
	<dsn alias="dashdb4" host="test@test.com" name="FOO" port="0000"/>
	</dsncollection>

  <databases>
	<database host="test@test.com" name="FOO" port="0000"/>
	</databases>

</configuration>
```

Include ibm_db in your cargo.toml with latest version from [Crates.io](https://crates.io/crates/ibm_db)

OR 

Simply include this project in your RUST project.

#### NOTE:

In case it is not already set, add the path of the CLI Driver downloaded as above to your Path on
Windows/LINUX/MACOS environment variable i.e. IBM_DB_HOME, PATH and LD_LIBRARY_PATH or DYLD_LIBRARY_PATH depending on Windows/LINUX/MACOS
e.g:
```
set PATH = C:/IBM/IBM_DATA_SERVER_DRIVER/clidriver/bin
set IBM_DB_HOME = C:/IBM/IBM_DATA_SERVER_DRIVER/clidriver
(or)
export LD_LIBRARY_PATH = /IBM/IBM_DATA_SERVER_DRIVER/clidriver/bin
export IBM_DB_HOME = /IBM/IBM_DATA_SERVER_DRIVER/clidriver
(or)
export DYLD_LIBRARY_PATH = /IBM/IBM_DATA_SERVER_DRIVER/clidriver/bin
export IBM_DB_HOME = /IBM/IBM_DATA_SERVER_DRIVER/clidriver
```

### <a name="Licenserequirements"></a> License requirements for connecting to databases

rust_ibm_db driver can connect to DB2 on Linux Unix and Windows without any additional license/s, however, connecting to databases on DB2 for z/OS or DB2 for i(AS400) Servers require either client side or server side license/s. The client side license would need to be copied under `license` folder of your `clidriver` installation directory and for activating server side license, you would need to purchase DB2 Connect Unlimited for System z® and DB2 Connect Unlimited Edition for System i®.

To know more about license and purchasing cost, please contact [IBM Customer Support](http://www-05.ibm.com/support/operations/zz/en/selectcountrylang.html).

To know more about server based licensing viz db2connectactivate, follow below links:
* [Activating the license certificate file for DB2 Connect Unlimited Edition](https://www.ibm.com/developerworks/community/blogs/96960515-2ea1-4391-8170-b0515d08e4da/entry/unlimited_licensing_in_non_java_drivers_using_db2connectactivate_utlility1?lang=en).
* [Unlimited licensing using db2connectactivate utility](https://www.ibm.com/developerworks/community/blogs/96960515-2ea1-4391-8170-b0515d08e4da/entry/unlimited_licensing_in_non_java_drivers_using_db2connectactivate_utlility1?lang=en.)

### How to run sample program:

To run the sample i.e. **main.rs** simply execute:- 

```
cargo run
```
#### You can also run other Sample Programs under examples folder using:
```
cargo run --package ibm_db --example <example_name i.e. connect or list_tables etc.>
e.g. cargo run --package ibm_db --example connect
```
## NOTE for MACOS:
If you get an error i.e. "dyld: Library not loaded: libdb2.dylib"
Run the following command(Where replace the <RUST_CRATE_LIB> with the path of your rust program root folder):

```
install_name_tool -change libdb2.dylib $IBM_DB_HOME/lib/libdb2.dylib <RUST_CRATE_LIB>/target/debug/ibm_db

```

<a name='contributing-to-the-ibm_db-RUST-project'></a>
## Contributing to the ibm_db RUST project

See [CONTRIBUTING](https://github.com/ibmdb/rust-ibm_db/blob/main/CONTRIBUTING.md)

```
The developer sign-off should include the reference to the DCO in remarks(example below):
DCO 1.1 Signed-off-by: Random J Developer <random@developer.org>
```